//! spamassassin model — spam filtering.

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub score: f64,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub subject: String,
    pub from: String,
    pub score: f64,
    pub is_spam: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub rules: Vec<Rule>,
    pub threshold: f64,
}

impl Default for Model {
    fn default() -> Self { Self {
        rules: vec![
            Rule { name: "BAYES_50".into(), score: 5.0, description: "Bayesian spam".into() },
            Rule { name: "HTML_MESSAGE".into(), score: 0.5, description: "HTML in message".into() },
        ],
        threshold: 5.0,
    } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn classify(&self, subject: &str, from: &str) -> Message {
        let mut score = 0.0;
        if subject.contains("viagra") { score += 5.0; }
        if from.contains("spam") { score += 3.0; }
        Message { subject: subject.into(), from: from.into(), score, is_spam: score >= self.threshold }
    }

    pub fn add_rule(&mut self, rule: Rule) { self.rules.push(rule); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classify_spam() {
        let s = Model::new();
        let msg = s.classify("Buy viagra now", "spam@x.com");
        assert!(msg.is_spam);
        let msg2 = s.classify("Hello", "friend@x.com");
        assert!(!msg2.is_spam);
    }
}
