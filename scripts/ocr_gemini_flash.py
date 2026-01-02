#!/usr/bin/env python3
"""OCR pipeline using Gemini 3 Flash via OpenRouter."""

import asyncio
import base64
import os
from pathlib import Path

import httpx
from dotenv import load_dotenv

load_dotenv()

OPENROUTER_API_KEY = os.getenv("OPENROUTER_API_KEY")
OPENROUTER_URL = "https://openrouter.ai/api/v1/chat/completions"
MODEL = "google/gemini-3-flash-preview"

SYSTEM_PROMPT = """You are an OCR assistant. Extract all text from the provided image and format it as clean HTML.

Rules:
- Preserve the document structure (headings, paragraphs, lists)
- Use semantic HTML tags (<h1>, <h2>, <p>, <ul>, <ol>, <li>, <strong>, <em>, etc.)
- For Tamil text, preserve the exact Tamil characters
- Do not add any wrapper <html>, <head>, or <body> tags - just the content
- Do not add any commentary or explanation, only output the HTML"""


async def encode_image(image_path: Path) -> str:
    """Encode image to base64."""
    with open(image_path, "rb") as f:
        return base64.b64encode(f.read()).decode("utf-8")


async def ocr_page(client: httpx.AsyncClient, image_path: Path) -> str:
    """Send image to Gemini 3 Flash for OCR."""
    base64_image = await encode_image(image_path)

    response = await client.post(
        OPENROUTER_URL,
        headers={
            "Authorization": f"Bearer {OPENROUTER_API_KEY}",
            "Content-Type": "application/json",
        },
        json={
            "model": MODEL,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": f"data:image/png;base64,{base64_image}"
                            },
                        },
                        {
                            "type": "text",
                            "text": "Extract all text from this page as HTML.",
                        },
                    ],
                },
            ],
            "max_tokens": 8192,
        },
        timeout=120.0,
    )

    response.raise_for_status()
    data = response.json()
    return data["choices"][0]["message"]["content"]


async def process_single_page(
    client: httpx.AsyncClient,
    semaphore: asyncio.Semaphore,
    page_path: Path,
    output_dir: Path,
) -> tuple[int, bool, str]:
    """Process a single page with semaphore for rate limiting."""
    page_num = int(page_path.stem.split("-")[1]) - 8  # offset: pdf page 9 = book page 1
    output_path = output_dir / f"page-{page_num:03d}.html"

    # Skip if already exists
    if output_path.exists():
        return page_num, True, "skipped (exists)"

    async with semaphore:
        print(f"Processing page {page_num} ({page_path.name})...")
        try:
            html_content = await ocr_page(client, page_path)
            output_path.write_text(html_content, encoding="utf-8")
            return page_num, True, "done"
        except Exception as e:
            return page_num, False, str(e)


async def process_pages(pages_dir: Path, output_dir: Path, concurrency: int = 5):
    """Process all pages in parallel."""
    output_dir.mkdir(parents=True, exist_ok=True)

    pages = sorted(pages_dir.glob("*.png"))
    print(f"Processing {len(pages)} pages with Gemini 3 Flash (concurrency={concurrency})")

    semaphore = asyncio.Semaphore(concurrency)

    async with httpx.AsyncClient() as client:
        tasks = [
            process_single_page(client, semaphore, page_path, output_dir)
            for page_path in pages
        ]
        results = await asyncio.gather(*tasks)

    # Print summary
    for page_num, success, msg in sorted(results):
        status = "✓" if success else "✗"
        print(f"  {status} page {page_num}: {msg}")


async def main():
    pages_dir = Path("data/cikaram/pages")
    output_dir = Path("data/cikaram/html-flash")
    await process_pages(pages_dir, output_dir, concurrency=5)
    print("Done!")


if __name__ == "__main__":
    asyncio.run(main())
