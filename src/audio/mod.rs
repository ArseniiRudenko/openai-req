use crate::FormRequest;
use std::io;
use std::path::{PathBuf};
use reqwest::multipart::{Form, Part};
use serde::{Serialize,Deserialize};
use crate::{AsyncTryFrom, file_to_part};
use async_trait::async_trait;

#[derive(Clone, Debug,Serialize,Deserialize)]
pub enum ResponseFormat{
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt
}

impl ToString for ResponseFormat {
    fn to_string(&self) -> String {
        match self {
            ResponseFormat::Json => "json".to_string(),
            ResponseFormat::Text => "text".to_string(),
            ResponseFormat::Srt => "srt".to_string(),
            ResponseFormat::VerboseJson => "verbose_json".to_string(),
            ResponseFormat::Vtt => "vtt".to_string()
        }
    }
}


#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct AudioResponse{
    pub text:String
}

///request that provides transcription for given audio file
/// # Usage example
///```
///
/// use std::path::PathBuf;
/// use openai_api::audio::TranscriptionRequest;
/// use openai_api::FormRequest;
///
///   let req =TranscriptionRequest::new(PathBuf::from("tests/Linus-linux.mp3"))
///         .language(Iso639_1::En);
///     let res = req.run(&client).await?;
///
/// ```
#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct TranscriptionRequest{
    file: PathBuf,
    model: String,
    prompt:Option<String>,
    response_format: Option<ResponseFormat>,
    temperature: Option<f64>,
    language: Option<Iso639_1>
}

impl FormRequest<AudioResponse> for TranscriptionRequest{
    const ENDPOINT: &'static str = "/audio/transcriptions";
}


#[async_trait]
impl AsyncTryFrom<TranscriptionRequest> for Form {
    type Error = io::Error;

    async fn try_from(transcription_request: TranscriptionRequest) -> Result<Self, Self::Error> {
        let mut form = Form::new();
        form = form.part("model", Part::text(transcription_request.model));
        form = form.part("file", file_to_part(&transcription_request.file).await?);

        if let Some(prompt) = transcription_request.prompt {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(response_format) = transcription_request.response_format {
            form = form.part("response_format", Part::text(response_format.to_string()));
        }

        if let Some(temperature) = transcription_request.temperature {
            form = form.part("temperature", Part::text(temperature.to_string()));
        }

        if let Some(language) = transcription_request.language {

            form = form.part("language", Part::text(language.to_string()));
        }

        Ok(form)
    }
}



impl TranscriptionRequest {

    /// minimal constructor is enough, you can run transcription by only providing file name,
    /// but quality will be significantly better if you at least specify source language
    pub fn new(file: PathBuf) -> Self {
        TranscriptionRequest {
            file,
            model: "whisper-1".to_string(),
            prompt: None,
            response_format: None,
            temperature: None,
            language: None
        }
    }

    pub fn with_model(file: PathBuf, model: String) -> Self {
        TranscriptionRequest {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None
        }
    }

    pub fn file(mut self, file: PathBuf) -> Self {
        self.file = file;
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn language(mut self, language: Iso639_1) -> Self {
        self.language = Some(language);
        self
    }

}

///request that provides translation to english for given audio file
/// # Usage example
///```
///
/// use std::path::PathBuf;
/// use openai_api::audio::TranslationRequest;
/// use openai_api::FormRequest;
/// let req = TranslationRequest::new(PathBuf::from("tests/Linus-linux.mp3"));
///     let res = req.run(&client).await?;
/// ```
#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct TranslationRequest{
    file: PathBuf,
    model: String,
    prompt:Option<String>,
    response_format: Option<ResponseFormat>,
    temperature: Option<f64>
}

impl FormRequest<AudioResponse> for TranslationRequest{
    const ENDPOINT: &'static str = "/audio/translations";
}


#[async_trait]
impl AsyncTryFrom<TranslationRequest> for Form {
    type Error = io::Error;

    async fn try_from(translation_request: TranslationRequest) -> Result<Self, Self::Error> {
        let mut form = Form::new();
        form = form.part("model", Part::text(translation_request.model));
        form = form.part("file", file_to_part(&translation_request.file).await?);

        if let Some(prompt) = translation_request.prompt {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(response_format) = translation_request.response_format {
            form = form.part("response_format", Part::text(response_format.to_string()));
        }

        if let Some(temperature) = translation_request.temperature {
            form = form.part("temperature", Part::text(temperature.to_string()));
        }

        Ok(form)
    }
}

impl TranslationRequest {
    pub fn new(file: PathBuf) -> Self {
        TranslationRequest {
            file,
            model:"whisper-1".to_string(),
            prompt: None,
            response_format: None,
            temperature: None
        }
    }

    pub fn with_model(file: PathBuf, model: String) -> Self {
        TranslationRequest {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None
        }
    }

    pub fn file(mut self, file: PathBuf) -> Self {
        self.file = file;
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
}


use strum_macros::Display;

#[derive(Clone, Debug, PartialEq, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum Iso639_1 {
    /// 639-2: aar, name: Afar (Afaraf)
    Aa,
    /// 639-2: abk, name: Abkhaz (аҧсуа бызшәа, аҧсшәа)
    Ab,
    /// 639-2: ave, name: Avestan (avesta)
    Ae,
    /// 639-2: afr, name: Afrikaans
    Af,
    /// 639-2: aka, name: Akan
    Ak,
    /// 639-2: amh, name: Amharic (አማርኛ)
    Am,
    /// 639-2: arg, name: Aragonese (aragonés)
    An,
    /// 639-2: ara, name: Arabic (العربية)
    Ar,
    /// 639-2: asm, name: Assamese (অসমীয়া)
    As,
    /// 639-2: ava, name: Avaric (авар мацӀ, магӀарул мацӀ)
    Av,
    /// 639-2: aym, name: Aymara (aymar aru)
    Ay,
    /// 639-2: aze, name: Azerbaijani (azərbaycan dili)
    Az,
    /// 639-2: bak, name: Bashkir (башҡорт теле)
    Ba,
    /// 639-2: bel, name: Belarusian (беларуская мова)
    Be,
    /// 639-2: bul, name: Bulgarian (български език)
    Bg,
    /// 639-2: bih, name: Bihari (भोजपुरी)
    Bh,
    /// 639-2: bis, name: Bislama
    Bi,
    /// 639-2: bam, name: Bambara (bamanankan)
    Bm,
    /// 639-2: ben, name: Bengali, Bangla (বাংলা)
    Bn,
    /// 639-2: bod, name: Tibetan Standard, Tibetan, Central (བོད་ཡིག)
    Bo,
    /// 639-2: bre, name: Breton (brezhoneg)
    Br,
    /// 639-2: bos, name: Bosnian (bosanski jezik)
    Bs,
    /// 639-2: cat, name: Catalan (català)
    Ca,
    /// 639-2: che, name: Chechen (нохчийн мотт)
    Ce,
    /// 639-2: cha, name: Chamorro (Chamoru)
    Ch,
    /// 639-2: cos, name: Corsican (corsu, lingua corsa)
    Co,
    /// 639-2: cre, name: Cree (ᓀᐦᐃᔭᐍᐏᐣ)
    Cr,
    /// 639-2: ces, name: Czech (čeština, český jazyk)
    Cs,
    /// 639-2: chu, name: Old Church Slavonic, Church Slavonic, Old Bulgarian (ѩзыкъ словѣньскъ)
    Cu,
    /// 639-2: chv, name: Chuvash (чӑваш чӗлхи)
    Cv,
    /// 639-2: cym, name: Welsh (Cymraeg)
    Cy,
    /// 639-2: dan, name: Danish (dansk)
    Da,
    /// 639-2: deu, name: German (Deutsch)
    De,
    /// 639-2: div, name: Divehi, Dhivehi, Maldivian (ދިވެހި)
    Dv,
    /// 639-2: dzo, name: Dzongkha (རྫོང་ཁ)
    Dz,
    /// 639-2: ewe, name: Ewe (Eʋegbe)
    Ee,
    /// 639-2: ell, name: Greek (modern) (ελληνικά)
    El,
    /// 639-2: eng, name: English
    En,
    /// 639-2: epo, name: Esperanto
    Eo,
    /// 639-2: spa, name: Spanish (Español)
    Es,
    /// 639-2: est, name: Estonian (eesti, eesti keel)
    Et,
    /// 639-2: eus, name: Basque (euskara, euskera)
    Eu,
    /// 639-2: fas, name: Persian (Farsi) (فارسی)
    Fa,
    /// 639-2: ful, name: Fula, Fulah, Pulaar, Pular (Fulfulde, Pulaar, Pular)
    Ff,
    /// 639-2: fin, name: Finnish (suomi, suomen kieli)
    Fi,
    /// 639-2: fij, name: Fijian (vosa Vakaviti)
    Fj,
    /// 639-2: fao, name: Faroese (føroyskt)
    Fo,
    /// 639-2: fra, name: French (français, langue française)
    Fr,
    /// 639-2: fry, name: Western Frisian (Frysk)
    Fy,
    /// 639-2: gle, name: Irish (Gaeilge)
    Ga,
    /// 639-2: gla, name: Scottish Gaelic, Gaelic (Gàidhlig)
    Gd,
    /// 639-2: glg, name: Galician (galego)
    Gl,
    /// 639-2: grn, name: Guaraní (Avañe'ẽ)
    Gn,
    /// 639-2: guj, name: Gujarati (ગુજરાતી)
    Gu,
    /// 639-2: glv, name: Manx (Gaelg, Gailck)
    Gv,
    /// 639-2: hau, name: Hausa ((Hausa) هَوُسَ)
    Ha,
    /// 639-2: heb, name: Hebrew (modern) (עברית)
    He,
    /// 639-2: hin, name: Hindi (हिन्दी, हिंदी)
    Hi,
    /// 639-2: hmo, name: Hiri Motu
    Ho,
    /// 639-2: hrv, name: Croatian (hrvatski jezik)
    Hr,
    /// 639-2: hat, name: Haitian, Haitian Creole (Kreyòl ayisyen)
    Ht,
    /// 639-2: hun, name: Hungarian (magyar)
    Hu,
    /// 639-2: hye, name: Armenian (Հայերեն)
    Hy,
    /// 639-2: her, name: Herero (Otjiherero)
    Hz,
    /// 639-2: ina, name: Interlingua
    Ia,
    /// 639-2: ind, name: Indonesian (Bahasa Indonesia)
    Id,
    /// 639-2: ile, name: Interlingue (Originally called Occidental; then Interlingue after WWII)
    Ie,
    /// 639-2: ibo, name: Igbo (Asụsụ Igbo)
    Ig,
    /// 639-2: iii, name: Nuosu (ꆈꌠ꒿ Nuosuhxop)
    Ii,
    /// 639-2: ipk, name: Inupiaq (Iñupiaq, Iñupiatun)
    Ik,
    /// 639-2: ido, name: Ido
    Io,
    /// 639-2: isl, name: Icelandic (Íslenska)
    Is,
    /// 639-2: ita, name: Italian (Italiano)
    It,
    /// 639-2: iku, name: Inuktitut (ᐃᓄᒃᑎᑐᑦ)
    Iu,
    /// 639-2: jpn, name: Japanese (日本語 (にほんご))
    Ja,
    /// 639-2: jav, name: Javanese (ꦧꦱꦗꦮ, Basa Jawa)
    Jv,
    /// 639-2: kat, name: Georgian (ქართული)
    Ka,
    /// 639-2: kon, name: Kongo (Kikongo)
    Kg,
    /// 639-2: kik, name: Kikuyu, Gikuyu (Gĩkũyũ)
    Ki,
    /// 639-2: kua, name: Kwanyama, Kuanyama (Kuanyama)
    Kj,
    /// 639-2: kaz, name: Kazakh (қазақ тілі)
    Kk,
    /// 639-2: kal, name: Kalaallisut, Greenlandic (kalaallisut, kalaallit oqaasii)
    Kl,
    /// 639-2: khm, name: Khmer (ខ្មែរ, ខេមរភាសា, ភាសាខ្មែរ)
    Km,
    /// 639-2: kan, name: Kannada (ಕನ್ನಡ)
    Kn,
    /// 639-2: kor, name: Korean (한국어)
    Ko,
    /// 639-2: kau, name: Kanuri
    Kr,
    /// 639-2: kas, name: Kashmiri (कश्मीरी, كشميري‎)
    Ks,
    /// 639-2: kur, name: Kurdish (Kurdî, كوردی‎)
    Ku,
    /// 639-2: kom, name: Komi (коми кыв)
    Kv,
    /// 639-2: cor, name: Cornish (Kernewek)
    Kw,
    /// 639-2: kir, name: Kyrgyz (Кыргызча, Кыргыз тили)
    Ky,
    /// 639-2: lat, name: Latin (latine, lingua latina)
    La,
    /// 639-2: ltz, name: Luxembourgish, Letzeburgesch (Lëtzebuergesch)
    Lb,
    /// 639-2: lug, name: Ganda (Luganda)
    Lg,
    /// 639-2: lim, name: Limburgish, Limburgan, Limburger (Limburgs)
    Li,
    /// 639-2: lin, name: Lingala (Lingála)
    Ln,
    /// 639-2: lao, name: Lao (ພາສາລາວ)
    Lo,
    /// 639-2: lit, name: Lithuanian (lietuvių kalba)
    Lt,
    /// 639-2: lub, name: Luba-Katanga (Tshiluba)
    Lu,
    /// 639-2: lav, name: Latvian (latviešu valoda)
    Lv,
    /// 639-2: mlg, name: Malagasy (fiteny malagasy)
    Mg,
    /// 639-2: mah, name: Marshallese (Kajin M̧ajeļ)
    Mh,
    /// 639-2: mri, name: Māori (te reo Māori)
    Mi,
    /// 639-2: mkd, name: Macedonian (македонски јазик)
    Mk,
    /// 639-2: mal, name: Malayalam (മലയാളം)
    Ml,
    /// 639-2: mon, name: Mongolian (Монгол хэл)
    Mn,
    /// 639-2: mar, name: Marathi (Marāṭhī) (मराठी)
    Mr,
    /// 639-2: msa, name: Malay (bahasa Melayu, بهاس ملايو‎)
    Ms,
    /// 639-2: mlt, name: Maltese (Malti)
    Mt,
    /// 639-2: mya, name: Burmese (ဗမာစာ)
    My,
    /// 639-2: nau, name: Nauruan (Dorerin Naoero)
    Na,
    /// 639-2: nob, name: Norwegian Bokmål (Norsk bokmål)
    Nb,
    /// 639-2: nde, name: Northern Ndebele (isiNdebele)
    Nd,
    /// 639-2: nep, name: Nepali (नेपाली)
    Ne,
    /// 639-2: ndo, name: Ndonga (Owambo)
    Ng,
    /// 639-2: nld, name: Dutch (Nederlands, Vlaams)
    Nl,
    /// 639-2: nno, name: Norwegian Nynorsk (Norsk nynorsk)
    Nn,
    /// 639-2: nor, name: Norwegian (Norsk)
    No,
    /// 639-2: nbl, name: Southern Ndebele (isiNdebele)
    Nr,
    /// 639-2: nav, name: Navajo, Navaho (Diné bizaad)
    Nv,
    /// 639-2: nya, name: Chichewa, Chewa, Nyanja (chiCheŵa, chinyanja)
    Ny,
    /// 639-2: oci, name: Occitan (occitan, lenga d'òc)
    Oc,
    /// 639-2: oji, name: Ojibwe, Ojibwa (ᐊᓂᔑᓈᐯᒧᐎᓐ)
    Oj,
    /// 639-2: orm, name: Oromo (Afaan Oromoo)
    Om,
    /// 639-2: ori, name: Oriya (ଓଡ଼ିଆ)
    Or,
    /// 639-2: oss, name: Ossetian, Ossetic (ирон æвзаг)
    Os,
    /// 639-2: pan, name: (Eastern) Punjabi (ਪੰਜਾਬੀ)
    Pa,
    /// 639-2: pli, name: Pāli (पाऴि)
    Pi,
    /// 639-2: pol, name: Polish (język polski, polszczyzna)
    Pl,
    /// 639-2: pus, name: Pashto, Pushto (پښتو)
    Ps,
    /// 639-2: por, name: Portuguese (Português)
    Pt,
    /// 639-2: que, name: Quechua (Runa Simi, Kichwa)
    Qu,
    /// 639-2: roh, name: Romansh (rumantsch grischun)
    Rm,
    /// 639-2: run, name: Kirundi (Ikirundi)
    Rn,
    /// 639-2: ron, name: Romanian (Română)
    Ro,
    /// 639-2: rus, name: Russian (Русский)
    Ru,
    /// 639-2: kin, name: Kinyarwanda (Ikinyarwanda)
    Rw,
    /// 639-2: san, name: Sanskrit (Saṁskṛta) (संस्कृतम्)
    Sa,
    /// 639-2: srd, name: Sardinian (sardu)
    Sc,
    /// 639-2: snd, name: Sindhi (सिन्धी, سنڌي، سندھی‎)
    Sd,
    /// 639-2: sme, name: Northern Sami (Davvisámegiella)
    Se,
    /// 639-2: sag, name: Sango (yângâ tî sängö)
    Sg,
    /// 639-2: sin, name: Sinhalese, Sinhala (සිංහල)
    Si,
    /// 639-2: slk, name: Slovak (slovenčina, slovenský jazyk)
    Sk,
    /// 639-2: slv, name: Slovene (slovenski jezik, slovenščina)
    Sl,
    /// 639-2: smo, name: Samoan (gagana fa'a Samoa)
    Sm,
    /// 639-2: sna, name: Shona (chiShona)
    Sn,
    /// 639-2: som, name: Somali (Soomaaliga, af Soomaali)
    So,
    /// 639-2: sqi, name: Albanian (Shqip)
    Sq,
    /// 639-2: srp, name: Serbian (српски језик)
    Sr,
    /// 639-2: ssw, name: Swati (SiSwati)
    Ss,
    /// 639-2: sot, name: Southern Sotho (Sesotho)
    St,
    /// 639-2: sun, name: Sundanese (Basa Sunda)
    Su,
    /// 639-2: swe, name: Swedish (svenska)
    Sv,
    /// 639-2: swa, name: Swahili (Kiswahili)
    Sw,
    /// 639-2: tam, name: Tamil (தமிழ்)
    Ta,
    /// 639-2: tel, name: Telugu (తెలుగు)
    Te,
    /// 639-2: tgk, name: Tajik (тоҷикӣ, toçikī, تاجیکی‎)
    Tg,
    /// 639-2: tha, name: Thai (ไทย)
    Th,
    /// 639-2: tir, name: Tigrinya (ትግርኛ)
    Ti,
    /// 639-2: tuk, name: Turkmen (Türkmen, Түркмен)
    Tk,
    /// 639-2: tgl, name: Tagalog (Wikang Tagalog)
    Tl,
    /// 639-2: tsn, name: Tswana (Setswana)
    Tn,
    /// 639-2: ton, name: Tonga (Tonga Islands) (faka Tonga)
    To,
    /// 639-2: tur, name: Turkish (Türkçe)
    Tr,
    /// 639-2: tso, name: Tsonga (Xitsonga)
    Ts,
    /// 639-2: tat, name: Tatar (татар теле, tatar tele)
    Tt,
    /// 639-2: twi, name: Twi
    Tw,
    /// 639-2: tah, name: Tahitian (Reo Tahiti)
    Ty,
    /// 639-2: uig, name: Uyghur (ئۇيغۇرچە‎, Uyghurche)
    Ug,
    /// 639-2: ukr, name: Ukrainian (Українська)
    Uk,
    /// 639-2: urd, name: Urdu (اردو)
    Ur,
    /// 639-2: uzb, name: Uzbek (Oʻzbek, Ўзбек, أۇزبېك‎)
    Uz,
    /// 639-2: ven, name: Venda (Tshivenḓa)
    Ve,
    /// 639-2: vie, name: Vietnamese (Tiếng Việt)
    Vi,
    /// 639-2: vol, name: Volapük
    Vo,
    /// 639-2: wln, name: Walloon (walon)
    Wa,
    /// 639-2: wol, name: Wolof (Wollof)
    Wo,
    /// 639-2: xho, name: Xhosa (isiXhosa)
    Xh,
    /// 639-2: yid, name: Yiddish (ייִדיש)
    Yi,
    /// 639-2: yor, name: Yoruba (Yorùbá)
    Yo,
    /// 639-2: zha, name: Zhuang, Chuang (Saɯ cueŋƅ, Saw cuengh)
    Za,
    /// 639-2: zho, name: Chinese (中文 (Zhōngwén), 汉语, 漢語)
    Zh,
    /// 639-2: zul, name: Zulu (isiZulu)
    Zu,
}



