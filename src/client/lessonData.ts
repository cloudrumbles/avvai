export interface VocabularyItem {
  tamil: string;
  english: string;
  phonetic?: string;
  extra?: string | null;
}

export interface Section {
  type: 'text' | 'vocabulary' | 'exercise' | 'examples';
  heading: string;
  content?: string;
  items: VocabularyItem[];
}

export interface Page {
  page_number: number;
  title: string;
  sections: Section[];
  summary?: string;
}

export const lessonData = {
  course_id: "a011",
  lesson_id: "a0111",
  title: "காப்பிய இலக்கியம்",
  subtitle: "Epic Literature",
  pages: [
    {
      page_number: 1,
      title: "1.0 பாட முன்னுரை (Lesson Introduction)",
      sections: [
        {
          type: "text",
          heading: "Introduction to Epics in Tamil Literature",
          content: "இலக்கிய உலகில் காப்பியம் ஒரு தனி இடத்தைப் பெற்றுள்ளது. இதனைச் செவ்விலக்கிய வகையில் (Classical Literature) அடக்குவர். இலக்கிய வளம் நிறைந்த பழமையான மொழிகளில் முதல் இலக்கியம் காப்பியமாக அமைவதைக் காணலாம்.",
          items: [
            {
              tamil: "காப்பியம்",
              english: "Epic",
              phonetic: "Kāppiyam",
              extra: "Refers to a grand narrative poem or literature."
            },
            {
              tamil: "செவ்விலக்கியம்",
              english: "Classical Literature",
              phonetic: "Cevvilakkiyam",
              extra: "Refers to high-quality ancient literary works."
            }
          ]
        },
        {
          type: "text",
          heading: "Heroic Age and Heroic Songs",
          content: "சங்க இலக்கியமான புறநானூறும், பதிற்றுப்பத்தும் பத்துப்பாட்டில் பல பாடல்களும் இத்தகைய வீரயுகப் பாடல்கள்தாம். சீன மொழியிலும் இத்தகைய உதிரிப் பாடல்களே வீரயுகத்தில் எழுந்துள்ளன.",
          items: [
            {
              tamil: "புறநானூறு",
              english: "Purananuru",
              phonetic: "Puṟanāṉūṟu",
              extra: "A Sangam anthology concerning secular life and heroism."
            },
            {
              tamil: "பத்துப்பாட்டு",
              english: "Pattupattu",
              phonetic: "Pattuppāṭṭu",
              extra: "The Ten Idylls; a collection of ancient Tamil poems."
            }
          ]
        },
        {
          type: "text",
          heading: "The Rise of Silappathikaram",
          content: "தமிழ் இலக்கிய வரலாற்றில் வீரயுகத்தை அடுத்துத்தான் காப்பியக் காலம் தொடங்குகிறது. இக்காப்பிய எழுச்சிக்கு வித்திட்டவர் இளங்கோ அடிகள் ஆவார். இவர் எழுதிய காப்பியம் சிலப்பதிகாரம் ஆகும். தமிழில் தோன்றிய முதல் காப்பியமே சிலப்பதிகாரம்தான்.",
          items: [
            {
              tamil: "சிலப்பதிகாரம்",
              english: "Silappathikaram",
              phonetic: "Cilappatikāram",
              extra: "The first major Tamil epic, 'The Tale of the Anklet'."
            },
            {
              tamil: "இளங்கோ அடிகள்",
              english: "Ilango Adigal",
              phonetic: "Iḷaṅkō Aṭikaḷ",
              extra: "The author of the epic Silappathikaram."
            }
          ]
        }
      ],
      summary: "This page introduces the concept of epics in literature, distinguishes between the Heroic Age of individual songs and the later Epic era in Tamil history, and identifies Silappathikaram as the first Tamil epic."
    },
    {
      page_number: 2,
      title: "1.1 காப்பியம் (Epic)",
      sections: [
        {
          type: "text",
          heading: "அறிமுகம் (Introduction)",
          content: "காப்பியம் என்றால் என்ன? இந்தச் சொல்லின் பொருள் என்ன? இச்சொல் விளக்கும் இலக்கியம் எத்தகையது? ஒருவகையில் சிந்தாமணி, சிலப்பதிகாரம் போன்றவை கதைப்பாடல்கள் என்பதை நாம் அறிவோம். இன்னொரு நிலையில் ‘காப்பியம்’ என்றால் என்ன? இந்தச் சொல் எங்கிருந்து வந்தது? இதன் அடிப்படை பொருள் யாது? இதற்கு விடை காண்பதே நமது நோக்கம்.",
          items: []
        },
        {
          type: "text",
          heading: "சொல் விளக்கம் (Etymology/Definition)",
          content: "வடமொழியில் காவ்யா என்றால் பாட்டு என்பது பொருள். கவியால் படைக்கப்படுவன அனைத்தும் காவியமே. எனவே காவ்யா - காவியம் - காப்பியம் என ஆகியது என்பர். தமிழில் தொல்காப்பியம், காப்பியக் குடி போன்ற பெயர்கள் காணப்படுகின்றன. இவை காப்பு + இயம் என்ற சொற்களின் சேர்க்கையாகக் கருதப்படுகின்றன. பழமரபுகளைக் காப்பது காப்பியம் எனக் கருத இடம் உண்டு.",
          items: [
            {
              tamil: "காவ்யா",
              english: "Song/Poem",
              phonetic: "Kavya",
              extra: "Sanskrit origin"
            },
            {
              tamil: "காப்பு + இயம்",
              english: "Protecting + that which is said",
              phonetic: "Kaappu + Iyam",
              extra: "Etymology of the Tamil word Kaappiyam"
            }
          ]
        },
        {
          type: "vocabulary",
          heading: "ஆங்கில இலக்கிய ஒப்புமை (English Literature Comparison)",
          content: "ஆங்கிலச் சொல்லான Epic என்பதும் ‘epo’ என்ற கிரேக்கச் சொல்லின் ஆக்கமாகக் கருதப்படுகிறது.",
          items: [
            {
              tamil: "ஈப்போ (epo)",
              english: "to tell",
              phonetic: "epo",
              extra: "Greek root"
            },
            {
              tamil: "ஈப்போஸ் (epos)",
              english: "anything to tell",
              phonetic: "epos",
              extra: "Greek root"
            }
          ]
        }
      ],
      summary: "This page explains the linguistic origin and definition of the Tamil word 'Kaappiyam' (Epic)."
    },
    {
      page_number: 3,
      title: "1.2 காப்பிய வகை (Types of Epics)",
      sections: [
        {
          type: "text",
          heading: "Introduction",
          content: "An introduction to the major and minor Tamil epics, including religious and modern narrative poems.",
          items: []
        },
        {
          type: "vocabulary",
          heading: "மேலை இலக்கியக் காப்பிய வகை (Western Literary Epics)",
          content: "Classification of epics originating from ancient languages like Greek, Latin, and Babylonian.",
          items: [
            {
              tamil: "முன்முறைக் காப்பியம்",
              english: "Primitive or Oral Epic",
              phonetic: "Munmurai Kappiyam",
              extra: "Based on oral traditions"
            },
            {
              tamil: "வழிமுறை அல்லது கலைக் காப்பியம்",
              english: "Secondary or Literary Epic",
              phonetic: "Vazhimurai allathu Kalaik Kappiyam",
              extra: "Involves poet's imagination"
            },
            {
              tamil: "வீரயுகக் காப்பியம்",
              english: "Chivalric Epic",
              phonetic: "Veerayuga Kappiyam",
              extra: "Deals with heroic deeds"
            }
          ]
        }
      ]
    },
    {
      page_number: 4,
      title: "1.2.2 வடமொழி & தமிழ் காப்பிய வகை",
      sections: [
        {
          type: "vocabulary",
          heading: "Types of Epics and Literature",
          content: "List of literary categories classified in traditional Tamil literature studies.",
          items: [
            {
              tamil: "இதிகாசம்",
              english: "Itihasa",
              phonetic: "Itikācam",
              extra: "Pre-historic period history"
            },
            {
              tamil: "மகாகாவியம்",
              english: "Great Epic",
              phonetic: "Makākāviyam",
              extra: "Deals with the four-fold goals of life"
            }
          ]
        },
        {
          type: "text",
          heading: "தமிழில் காப்பிய வகை (Epic Genres in Tamil)",
          content: "Tamil literature is famous for the Five Great Epics (Cilappatikaram, Manimekalai, etc.) and Five Minor Epics.",
          items: [
            {
              tamil: "ஐம்பெருங் காப்பியங்கள்",
              english: "The Five Great Epics",
              phonetic: "Aimperung kāppiyaṅkaḷ",
              extra: null
            },
            {
              tamil: "ஐஞ்சிறு காப்பியங்கள்",
              english: "The Five Minor Epics",
              phonetic: "Aiñciṟu kāppiyaṅkaḷ",
              extra: null
            }
          ]
        }
      ]
    }
  ]
};
