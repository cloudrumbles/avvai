package main

import (
	"net/http"

	"font-gallery/templates"
)

var fonts = []templates.Font{
	{
		ID:          "catamaran",
		NameTamil:   "கேட்டமரன்",
		NameEnglish: "Catamaran",
		Designer:    "Pria Ravichandran",
		Year:        "2015",
		Description: "Designed Tamil-first, later expanded to Latin. Clean geometric forms with excellent screen legibility.",
		SampleText:  "அகர முதல எழுத்தெல்லாம் ஆதி பகவன் முதற்றே உலகு",
		SampleMeta:  "திருக்குறள் 1 — திருவள்ளுவர்",
		FontFamily:  "'Catamaran', sans-serif",
		GoogleFont:  "Catamaran:wght@400;700;900",
		Category:    "sans-serif",
	},
	{
		ID:          "anek-tamil",
		NameTamil:   "அனேக் தமிழ்",
		NameEnglish: "Anek Tamil",
		Designer:    "Ek Type",
		Year:        "2022",
		Description: "First variable font designed for Tamil. Part of the Anek superfamily spanning 12 Indian scripts.",
		SampleText:  "கற்றதனால் ஆய பயனென்கொல் வாலறிவன் நற்றாள் தொழாஅர் எனின்",
		SampleMeta:  "திருக்குறள் 2 — திருவள்ளுவர்",
		FontFamily:  "'Anek Tamil', sans-serif",
		GoogleFont:  "Anek+Tamil:wght@400;600;800",
		Category:    "variable",
	},
	{
		ID:          "tiro-tamil",
		NameTamil:   "டைரோ தமிழ்",
		NameEnglish: "Tiro Tamil",
		Designer:    "Tiro Typeworks",
		Year:        "2021",
		Description: "Elegant serif with classical proportions. Designed for extended reading in print and screen.",
		SampleText:  "மலர்மிசை ஏகினான் மாணடி சேர்ந்தார் நிலமிசை நீடுவாழ் வார்",
		SampleMeta:  "திருக்குறள் 3 — திருவள்ளுவர்",
		FontFamily:  "'Tiro Tamil', serif",
		GoogleFont:  "Tiro+Tamil:ital,wght@0,400;1,400",
		Category:    "serif",
	},
	{
		ID:          "noto-sans-tamil",
		NameTamil:   "நோட்டோ சான்ஸ்",
		NameEnglish: "Noto Sans Tamil",
		Designer:    "Google & Monotype",
		Year:        "2014",
		Description: "Part of Google's mission to eliminate tofu. Comprehensive character coverage with multiple weights.",
		SampleText:  "வேண்டுதல் வேண்டாமை இலானடி சேர்ந்தார்க்கு யாண்டும் இடும்பை இல",
		SampleMeta:  "திருக்குறள் 4 — திருவள்ளுவர்",
		FontFamily:  "'Noto Sans Tamil', sans-serif",
		GoogleFont:  "Noto+Sans+Tamil:wght@400;600;800",
		Category:    "sans-serif",
	},
	{
		ID:          "noto-serif-tamil",
		NameTamil:   "நோட்டோ செரிஃப்",
		NameEnglish: "Noto Serif Tamil",
		Designer:    "Google & Monotype",
		Year:        "2020",
		Description: "Serif companion to Noto Sans Tamil. Refined strokes inspired by traditional Tamil calligraphy.",
		SampleText:  "இருள்சேர் இருவினையும் சேரா இறைவன் பொருள்சேர் புகழ்புரிந்தார் மாட்டு",
		SampleMeta:  "திருக்குறள் 5 — திருவள்ளுவர்",
		FontFamily:  "'Noto Serif Tamil', serif",
		GoogleFont:  "Noto+Serif+Tamil:wght@400;600;800",
		Category:    "serif",
	},
	{
		ID:          "pavanam",
		NameTamil:   "பவனம்",
		NameEnglish: "Pavanam",
		Designer:    "Enathu",
		Year:        "2017",
		Description: "Modern sans-serif optimized for digital screens. Clean lines with a contemporary feel.",
		SampleText:  "பொறிவாயில் ஐந்தவித்தான் பொய்தீர் ஒழுக்க நெறிநின்றார் நீடுவாழ் வார்",
		SampleMeta:  "திருக்குறள் 6 — திருவள்ளுவர்",
		FontFamily:  "'Pavanam', sans-serif",
		GoogleFont:  "Pavanam",
		Category:    "sans-serif",
	},
	{
		ID:          "kavivanar",
		NameTamil:   "கவிவாணர்",
		NameEnglish: "Kavivanar",
		Designer:    "Tharique Azeez",
		Year:        "2018",
		Description: "Handwriting-style font honoring poet M.A. Azeez (1948-2002). Organic, flowing strokes.",
		SampleText:  "தனக்குவமை இல்லாதான் தாள்சேர்ந்தார்க் கல்லால் மனக்கவலை மாற்றல் அரிது",
		SampleMeta:  "திருக்குறள் 7 — திருவள்ளுவர்",
		FontFamily:  "'Kavivanar', cursive",
		GoogleFont:  "Kavivanar",
		Category:    "handwriting",
	},
	{
		ID:          "meera-inimai",
		NameTamil:   "மீரா இனிமை",
		NameEnglish: "Meera Inimai",
		Designer:    "SMC & Hiran",
		Year:        "2016",
		Description: "Graceful curves with calligraphic influence. Balances tradition with modern readability.",
		SampleText:  "அறவாழி அந்தணன் தாள்சேர்ந்தார்க் கல்லால் பிறவாழி நீந்தல் அரிது",
		SampleMeta:  "திருக்குறள் 8 — திருவள்ளுவர்",
		FontFamily:  "'Meera Inimai', sans-serif",
		GoogleFont:  "Meera+Inimai",
		Category:    "sans-serif",
	},
}

func buildFontCSS() string {
	css := ""
	for _, f := range fonts {
		css += ".font-" + f.ID + " { font-family: " + f.FontFamily + "; }\n"
	}
	return css
}

func main() {
	// Serve static files
	fs := http.FileServer(http.Dir("static"))
	http.Handle("/static/", http.StripPrefix("/static/", fs))

	// Dynamic font CSS
	http.HandleFunc("/fonts.css", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/css")
		w.Write([]byte(buildFontCSS()))
	})

	// Main gallery page
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			http.NotFound(w, r)
			return
		}
		templates.Gallery(fonts).Render(r.Context(), w)
	})

	// HTMX endpoint for font detail expansion
	http.HandleFunc("/font/", func(w http.ResponseWriter, r *http.Request) {
		id := r.URL.Path[len("/font/"):]
		for _, f := range fonts {
			if f.ID == id {
				templates.FontDetail(f).Render(r.Context(), w)
				return
			}
		}
		http.NotFound(w, r)
	})

	// HTMX endpoint for specimen size toggle
	http.HandleFunc("/specimen/", func(w http.ResponseWriter, r *http.Request) {
		id := r.URL.Path[len("/specimen/"):]
		size := r.URL.Query().Get("size")
		for _, f := range fonts {
			if f.ID == id {
				templates.Specimen(f, size).Render(r.Context(), w)
				return
			}
		}
		http.NotFound(w, r)
	})

	println("Tamil Font Gallery running at http://localhost:3004")
	http.ListenAndServe(":3004", nil)
}
