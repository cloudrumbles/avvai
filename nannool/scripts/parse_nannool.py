#!/usr/bin/env python3
"""Parse Nannool HTML and extract புணரியல் verses (151-257)."""

import re
import toml
from pathlib import Path

def parse_nannool_html(html_path: Path) -> list[dict]:
    """Extract verses from the Nannool HTML file."""
    content = html_path.read_text(encoding='utf-8')

    # Verse pattern: "151.	text..."
    verse_pattern = re.compile(r'(\d+)\.\s*(.+?)(?=\n\s*\d+\.\s*|---|</body>|$)', re.DOTALL)
    
    # Headers to strip if they appear at the end of a verse
    headers = [
        "பொதுப்புணர்ச்சி",
        "உயிரீற்றுச் சிறப்புப் புணர்ச்சி",
        "உயிரீற்று முன் வல்லினம்",
        "அகர வீற்றுச் சிறப்புவிதி",
        "ஆகார வீற்றுச் சிறப்புவிதி",
        "இகர வீற்றுச் சிறப்புவிதி",
        "ஈகார வீற்றுச் சிறப்புவிதி",
        "முற்றுகர வீற்றுச் சிறப்புவிதி",
        "குற்றுகர வீற்றுச் சிறப்புவிதி",
        "ஊகார வீற்றுச் சிறப்புவிதி",
        "ஏகார வீற்றுச் சிறப்புவிதி",
        "ஐகார வீற்றுச் சிறப்புவிதி",
        "மெய்யீற்றின் முன் உயிர்",
        "மெய்யீற்றின் முன் மெய்",
        "ணகர னகரவீறு",
        "மகரவீறு",
        "ய ர ழ வீறு",
        "லகர ளகர வீறு",
        "வகர வீறு",
        "வருமொழித் தகர நகரத் திரிபு",
        "வேற்றுமைப் பொருட் புணர்ச்சியை உருபு புணர்ச்சியோடு மாட்டெறிதல்",
        "புணரியல்களுக்குப் புறனடை",
        "உருபுகள்",
        "சாரியை",
        "உருபு புணர்ச்சிக்குச் சிறப்புவிதி",
        "புறனடை"
    ]

    verses = []
    for match in verse_pattern.finditer(content):
        num = int(match.group(1))
        if 151 <= num <= 257:
            raw_text = match.group(2).strip()
            
            # Remove any trailing headers in tags
            raw_text = re.split(r'<(strong|center|STRONG|CENTER|b|B)>', raw_text, flags=re.IGNORECASE)[0]
            
            # Remove all HTML tags
            clean_text = re.sub(r'<[^>]+>', ' ', raw_text)
            
            # Clean up whitespace
            clean_text = re.sub(r'\s+', ' ', clean_text).strip()
            
            # Remove any trailing section headers that might have been plain text
            for header in headers:
                if clean_text.endswith(header):
                    clean_text = clean_text[: -len(header)].strip()
            
            verses.append({
                'number': num,
                'text': clean_text
            })

    return verses

def get_section(num: int) -> str:
    """Get the section name for a verse number."""
    if 151 <= num <= 203:
        return "உயிரீற்றுப் புணரியல்"
    elif 204 <= num <= 239:
        return "மெய்யீற்றுப் புணரியல்"
    elif 240 <= num <= 257:
        return "உருபு புணரியல்"
    return ""

def merge_with_existing(verses: list[dict], existing_path: Path) -> list[dict]:
    """Merge newly parsed verses with existing ones to preserve meanings/commentary."""
    if not existing_path.exists():
        return verses
        
    try:
        existing_data = toml.loads(existing_path.read_text(encoding='utf-8'))
        existing_map = {v['number']: v for v in existing_data.get('verse', [])}
        
        for v in verses:
            if v['number'] in existing_map:
                existing = existing_map[v['number']]
                if 'meaning' in existing and existing['meaning']:
                    v['meaning'] = existing['meaning']
                if 'commentary' in existing and existing['commentary']:
                    v['commentary'] = existing['commentary']
    except Exception as e:
        print(f"Warning: Could not parse existing TOML: {e}")
        
    return verses

def to_toml_str(verses: list[dict]) -> str:
    """Convert verses to TOML format manually for better control over layout."""
    lines = ['# நன்னூல் புணரியல் (Sandhi Rules) - Verses 151-257', '']
    
    current_section = ""
    for v in verses:
        section = get_section(v['number'])
        if section != current_section:
            if current_section:
                lines.append('')
            lines.append(f'# {section}')
            current_section = section
            
        lines.append('[[verse]]')
        lines.append(f'number = {v["number"]}')
        lines.append(f'section = "{section}"')
        
        text = v['text'].replace('\\', '\\\\').replace('"', '\\"')
        lines.append(f'text = "{text}"')
        
        if 'meaning' in v and v['meaning']:
            meaning = v['meaning'].replace('\\', '\\\\').replace('"', '\\"')
            lines.append(f'meaning = "{meaning}"')
            
        if 'commentary' in v and v['commentary']:
            commentary = v['commentary'].replace('\\', '\\\\').replace('"', '\\"')
            lines.append(f'commentary = "{commentary}"')
            
        lines.append('')
        
    return '\n'.join(lines)

if __name__ == '__main__':
    script_path = Path(__file__).resolve()
    html_path = script_path.parent.parent / 'data' / 'nannool' / 'nannool-full.html'
    output_path = html_path.parent / 'punariyar.toml'
    
    if not html_path.exists():
        html_path = Path('data/nannool/nannool-full.html')
        output_path = Path('data/nannool/punariyar.toml')

    verses = parse_nannool_html(html_path)
    verses = merge_with_existing(verses, output_path)
    
    print(f"Found {len(verses)} புணரியல் verses")
    
    toml_content = to_toml_str(verses)
    output_path.write_text(toml_content, encoding='utf-8')
    print(f"Written to {output_path}")
