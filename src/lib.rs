#![feature(test)]

extern crate test;

use lazy_static::lazy_static;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug)]
struct RuleRe {
    from: Regex,
    to: &'static str
}

type RuleFnType = fn(&str) -> String;

#[derive(Clone)]
struct RuleFn {
    function: RuleFnType
}

#[derive(Clone)]
enum Rule {
    Regex(RuleRe),
    Function(RuleFn)
}

impl Rule {
    pub fn new_re(from: Regex, to: &'static str) -> Self {
        Rule::Regex(RuleRe { from, to })
    }
    pub fn new_fn(function: RuleFnType) -> Self {
        Rule::Function(RuleFn { function })
    }
}

macro_rules! rule_re {
    ($from:expr => $to:expr) => {
        Rule::new_re(Regex::new($from).unwrap(), $to)
    }
}
macro_rules! rule_fn {
    ($fun:expr) => {
        Rule::new_fn($fun)
    }
}

fn remove_duplicate_characters(data: &str) -> String {
    data.chars().dedup().collect()
}

lazy_static! {
    static ref RulesRussian : Vec<Rule> = vec![
        rule_re! { r"[ЪЬ]" => "" },
        rule_re! { r"[^А-Я]" => "" },
        rule_fn! { remove_duplicate_characters },

        rule_re! { "ЙО|ИО|ЙЕ|ИЕ" => "И" },
        rule_re! { "[ОЫЯ]" => "А" },
        rule_re! { "[ЕЁЭ]" => "И" },
        rule_re! { "Ю" => "У" },

        rule_re! { "Б(Б|В|Г|Д|Ж|З|Й|К|П|С|Т|Ф|Х|Ц|Ч|Ш|Щ)" => "П$1" },
        rule_re! { "Б$" => "П" },

        rule_re! { "З(Б|В|Г|Д|Ж|З|Й|К|П|С|Т|Ф|Х|Ц|Ч|Ш|Щ)" => "С$1" },
        rule_re! { "З$" => "С" },

        rule_re! { "Д(Б|В|Г|Д|Ж|З|Й|К|П|С|Т|Ф|Х|Ц|Ч|Ш|Щ)" => "Т$1" },
        rule_re! { "Д$" => "Т" },

        rule_re! { "В(Б|В|Г|Д|Ж|З|Й|К|П|С|Т|Ф|Х|Ц|Ч|Ш|Щ)" => "Ф$1" },
        rule_re! { "В$" => "Ф" },

        rule_re! { "Г(Б|В|Г|Д|Ж|З|Й|К|П|С|Т|Ф|Х|Ц|Ч|Ш|Щ)" => "К$1" },
        rule_re! { "Г$" => "К" },

        rule_re! { "ТС|ДС" => "Ц" },

        rule_fn! { remove_duplicate_characters },
    ];

    static ref RulesUkrainian : Vec<Rule> = vec![
        rule_re! { r"[ІЇ]" => "И" },
    ];
}

enum Language {
    Russian,
    Ukrainian,
}

struct Metaphone {
    rules: Vec<Rule>
}

impl Metaphone {
    pub fn new(lang: Language) -> Self {
        match lang {
            Language::Russian => Metaphone { rules: RulesRussian.to_vec() },
            Language::Ukrainian => Metaphone { rules: ([&RulesUkrainian.to_vec()[..], &RulesRussian.to_vec()[..]].concat()) },
        }
    }

    pub fn get(&self, word: &str) -> String {
        let mut result = word.to_uppercase();

        for rule in &self.rules {
            result = match rule {
                Rule::Regex(re) => re.from.replace_all(&result, re.to).to_string(),
                Rule::Function(h) => (h.function)(&result)
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_ru(b: &mut Bencher) {
        let metaphone = Metaphone::new(Language::Russian);
        b.iter(|| metaphone.get("Шварценеггер"));
    }

    #[test]
    fn test_ru() {
        let metaphone = Metaphone::new(Language::Russian);

        assert_eq!(metaphone.get("Шварценеггер"), "ШВАРЦИНИГИР");
        assert_eq!(metaphone.get("Взвешенный"), "ФСВИШИНАЙ");
        assert_eq!(metaphone.get("Аффилированный"), "АФИЛИРАВАНАЙ");
        assert_eq!(metaphone.get("Воспользовавшемуся"), "ВАСПАЛЗАВАФШИМУСА");
        assert_eq!(metaphone.get("Предшествовавшими"), "ПРИТШИСТВАВАФШИМИ");
        assert_eq!(metaphone.get("Нововодолажского"), "НАВАВАДАЛАЖСКАГА");
        assert_eq!(metaphone.get("Неиссякаемый"), "НИСАКАИМАЙ");
        assert_eq!(metaphone.get("Дифференцированная"), "ДИФИРИНЦИРАВАНА");
        assert_eq!(metaphone.get("Мальденштам"), "МАЛДИНШТАМ");
        assert_eq!(metaphone.get("Верблюд"), "ВИРБЛУТ");
    }

    #[test]
    fn test_ua() {
        let metaphone = Metaphone::new(Language::Ukrainian);
        assert_eq!(metaphone.get("Взвінчений"), "ФСВИНЧИНИЙ");
    }
}