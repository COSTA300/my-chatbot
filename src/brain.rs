use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Intent {
    Greeting,
    AskIdentity,
    AskHealth,
    ShareName(String),
    ExpressDistress,
    AskAdvice(Topic),
    AskQuestion(Topic),
    ShareExperience(Topic),
    Farewell,
    Gratitude,
    Unknown(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Topic {
    MentalHealth,
    Career,
    Relationships,
    Technology,
    Finance,
    Health,
    Learning,
    General,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
    Curious,
}

pub struct ConversationContext {
    pub last_topic: Option<Topic>,
    pub turn_count: u32,
    pub sentiment_history: Vec<Sentiment>,
}

pub struct Brain {
    pub user_name: Option<String>,
    pub context: ConversationContext,
    knowledge_base: HashMap<String, Vec<String>>,
}

impl Brain {
    pub fn new() -> Self {
        let mut kb = HashMap::new();

        kb.insert("career".to_string(), vec![
            "Start by identifying your strengths — what tasks feel effortless to you but hard for others?".to_string(),
            "Networking is underrated. 70% of jobs are filled through connections, not applications.".to_string(),
            "Build a portfolio of projects. Evidence beats credentials in most tech fields.".to_string(),
            "Learn to communicate your value clearly. Technical skill alone rarely gets you far.".to_string(),
        ]);

        kb.insert("mental_health".to_string(), vec![
            "Small consistent habits beat big dramatic changes. Start with just 5 minutes of journaling.".to_string(),
            "Physical movement is one of the most powerful mood regulators we know of.".to_string(),
            "You don't have to fix everything at once. Focus on the next small step.".to_string(),
            "Talking to someone — even a trusted friend — can reduce emotional weight significantly.".to_string(),
        ]);

        kb.insert("relationships".to_string(), vec![
            "Clear, honest communication prevents most conflicts before they start.".to_string(),
            "Listen to understand, not just to reply. Most people feel unheard.".to_string(),
            "Boundaries aren't walls — they're the terms under which you can genuinely show up.".to_string(),
            "People remember how you made them feel far longer than what you said.".to_string(),
        ]);

        kb.insert("technology".to_string(), vec![
            "The best way to learn a technology is to build something real with it — tutorials only get you so far.".to_string(),
            "Understand the fundamentals. Languages change, but algorithms, networking, and system design don't.".to_string(),
            "Cloud skills are highly valuable right now — AWS, Azure, and GCP are worth your time.".to_string(),
            "Read other people's code. Open source projects are the best classroom.".to_string(),
        ]);

        kb.insert("finance".to_string(), vec![
            "Pay yourself first — automate savings before you see the money.".to_string(),
            "An emergency fund of 3-6 months of expenses is the foundation of financial stability.".to_string(),
            "Compound interest works both for and against you. Start investing early, pay debt fast.".to_string(),
            "Track your spending for just 30 days — the awareness alone changes behavior.".to_string(),
        ]);

        kb.insert("learning".to_string(), vec![
            "Active recall beats passive review. Test yourself instead of re-reading.".to_string(),
            "Spaced repetition systems like Anki are scientifically proven to improve retention.".to_string(),
            "Teach what you learn to someone else — it forces clarity and reveals gaps.".to_string(),
            "Block deep work sessions. Distracted learning is mostly wasted time.".to_string(),
        ]);

        Brain {
            user_name: None,
            context: ConversationContext {
                last_topic: None,
                turn_count: 0,
                sentiment_history: Vec::new(),
            },
            knowledge_base: kb,
        }
    }

    pub fn understand_intents(&self, input: &str) -> Vec<Intent> {
        let normalized = input.to_lowercase();
        let mut intents = Vec::new();

        // --- Name extraction ---
        if let Some(idx) = normalized.find("my name is ") {
            let offset = idx + "my name is ".len();
            intents.push(Intent::ShareName(self.extract_first_word(&input[offset..])));
        } else if let Some(idx) = normalized.find("i am ") {
            let offset = idx + "i am ".len();
            let potential_name = self.extract_first_word(&input[offset..]);
            let lower = potential_name.to_lowercase();
            let excluded = ["happy","sad","tired","angry","good","bad","fine","okay","glad","not","trying","going","feeling"];
            if !excluded.contains(&lower.as_str()) {
                intents.push(Intent::ShareName(potential_name));
            }
        } else if let Some(idx) = normalized.find("call me ") {
            let offset = idx + "call me ".len();
            intents.push(Intent::ShareName(self.extract_first_word(&input[offset..])));
        }

        // --- Greetings ---
        if normalized.contains("hello") || normalized.contains("hi") || normalized.contains("hey") || normalized.contains("good morning") || normalized.contains("good evening") {
            intents.push(Intent::Greeting);
        }

        // --- Farewell ---
        if normalized.contains("bye") || normalized.contains("goodbye") || normalized.contains("see you") || normalized.contains("take care") {
            intents.push(Intent::Farewell);
        }

        // --- Gratitude ---
        if normalized.contains("thank") || normalized.contains("thanks") || normalized.contains("appreciate") {
            intents.push(Intent::Gratitude);
        }

        // --- Identity ---
        if normalized.contains("who are you") || normalized.contains("your name") || normalized.contains("what are you") {
            intents.push(Intent::AskIdentity);
        }

        // --- Health check ---
        if normalized.contains("how are you") || normalized.contains("how do you do") || normalized.contains("how you feeling") {
            intents.push(Intent::AskHealth);
        }

        // --- Topic detection helper ---
        let topic = self.detect_topic(&normalized);

        // --- Advice requests ---
        let advice_triggers = ["advice", "help me", "what should i", "how do i", "how can i", "what can i do", "should i", "suggest", "recommend", "tips for", "guide me"];
        if advice_triggers.iter().any(|t| normalized.contains(t)) {
            intents.push(Intent::AskAdvice(topic.clone()));
        }

        // --- Question detection ---
        let question_words = ["why", "what is", "what are", "how does", "explain", "tell me about", "what do you think", "do you know"];
        let is_question = input.trim_end().ends_with('?') || question_words.iter().any(|q| normalized.contains(q));
        if is_question && !intents.iter().any(|i| matches!(i, Intent::AskAdvice(_))) {
            intents.push(Intent::AskQuestion(topic.clone()));
        }

        // --- Distress ---
        let distress_words = ["tired","sad","tough","depressed","struggling","hard time","overwhelmed","lonely","anxious","stressed","burnt out","burnout","hopeless","lost","can't cope"];
        if distress_words.iter().any(|w| normalized.contains(w)) {
            intents.push(Intent::ExpressDistress);
        }

        // --- Sharing experience ---
        let share_triggers = ["i feel", "i think", "i believe", "i've been", "i have been", "lately i", "recently i", "i'm going through"];
        if share_triggers.iter().any(|t| normalized.contains(t)) && !intents.iter().any(|i| matches!(i, Intent::ExpressDistress)) {
            intents.push(Intent::ShareExperience(topic));
        }

        if intents.is_empty() {
            intents.push(Intent::Unknown(input.to_string()));
        }

        intents
    }

    fn detect_topic(&self, text: &str) -> Topic {
        let career_words = ["job","career","work","interview","resume","cv","salary","promotion","boss","colleague","profession","employment"];
        let mental_words = ["anxiety","stress","depress","mental health","therapy","counseling","mood","emotion","feeling","cope","wellbeing"];
        let rel_words = ["relationship","friend","partner","family","love","breakup","marriage","communication","trust","conflict"];
        let tech_words = ["code","programming","software","computer","tech","developer","cloud","api","rust","python","java","database","algorithm"];
        let finance_words = ["money","finance","budget","invest","debt","saving","loan","expense","income","salary","bank"];
        let learn_words = ["study","learn","school","university","exam","course","skill","knowledge","education","degree"];
        let health_words = ["health", "sick", "doctor", "hospital", "illness", "disease", "pain", "injury", "medicine", "exercise", "diet"];

        if career_words.iter().any(|w| text.contains(w)) { Topic::Career }
        else if mental_words.iter().any(|w| text.contains(w)) { Topic::MentalHealth }
        else if rel_words.iter().any(|w| text.contains(w)) { Topic::Relationships }
        else if tech_words.iter().any(|w| text.contains(w)) { Topic::Technology }
        else if finance_words.iter().any(|w| text.contains(w)) { Topic::Finance }
        else if learn_words.iter().any(|w| text.contains(w)) { Topic::Learning }
        else if health_words.iter().any(|w| text.contains(w)) { Topic::Health }
        else { Topic::General }
    }

    fn get_advice(&self, topic: &Topic) -> String {
        let key = match topic {
            Topic::Career => "career",
            Topic::MentalHealth => "mental_health",
            Topic::Relationships => "relationships",
            Topic::Technology => "technology",
            Topic::Finance => "finance",
            Topic::Learning => "learning",
            Topic::Health => "mental_health",
            Topic::General => "career", // fallback
        };

        if let Some(tips) = self.knowledge_base.get(key) {
            // Rotate based on turn count so it doesn't repeat
            let idx = self.context.turn_count as usize % tips.len();
            tips[idx].clone()
        } else {
            "Tell me more — the more context you share, the better I can help.".to_string()
        }
    }

    fn detect_sentiment(&self, input: &str) -> Sentiment {
        let normalized = input.to_lowercase();
        let positive = ["great","amazing","happy","excited","love","wonderful","good","awesome","fantastic","glad","enjoying"];
        let negative = ["bad","terrible","awful","hate","horrible","worst","difficult","struggling","tired","sad","depressed","overwhelmed"];
        let curious = ["why","how","what","curious","wondering","interested","explain","tell me"];

        if positive.iter().any(|w| normalized.contains(w)) { Sentiment::Positive }
        else if negative.iter().any(|w| normalized.contains(w)) { Sentiment::Negative }
        else if curious.iter().any(|w| normalized.contains(w)) { Sentiment::Curious }
        else { Sentiment::Neutral }
    }

    pub fn generate_response(&self, intents: Vec<Intent>) -> String {
        let mut parts: Vec<String> = Vec::new();
        let name_str = self.user_name.as_deref().unwrap_or("");

        for intent in &intents {
            match intent {
                Intent::Greeting => {
                    if !name_str.is_empty() {
                        parts.push(format!("Hey {}! Good to see you again.", name_str));
                    } else {
                        let greetings = ["Hey there! Great to meet you.", "Hello! I'm glad you stopped by.", "Hi! Ready to chat."];
                        parts.push(greetings[self.context.turn_count as usize % greetings.len()].to_string());
                    }
                }
                Intent::Farewell => {
                    let farewell = if !name_str.is_empty() {
                        format!("Take care, {}! Feel free to come back anytime.", name_str)
                    } else {
                        "Take care! Come back anytime you want to talk.".to_string()
                    };
                    parts.push(farewell);
                }
                Intent::Gratitude => {
                    parts.push("Always happy to help! That's what I'm here for.".to_string());
                }
                Intent::AskIdentity => {
                    parts.push("I'm a conversational AI written in Rust. I'm here to listen, have a real conversation, and share knowledge or advice when I can.".to_string());
                }
                Intent::AskHealth => {
                    parts.push("I'm running smoothly! More importantly — how are YOU doing?".to_string());
                }
                Intent::ShareName(name) => {
                    parts.push(format!("It's great to properly meet you, {}! I'll remember that.", name));
                }
                Intent::ExpressDistress => {
                    let empathy = if !name_str.is_empty() {
                        format!("I hear you, {}. That sounds genuinely tough.", name_str)
                    } else {
                        "I hear you. That sounds genuinely tough.".to_string()
                    };
                    let advice = self.get_advice(&Topic::MentalHealth);
                    parts.push(format!("{} You're not alone in feeling this way. Here's something that might help: {}", empathy, advice));
                    parts.push("Would you like to talk more about what's going on?".to_string());
                }
                Intent::AskAdvice(topic) => {
                    let intro = match topic {
                        Topic::General => "I can share some thoughts on that.".to_string(),
                        _ => format!("On the topic of {}, here's what I'd suggest:", Self::topic_name(topic)),
                    };
                    let advice = self.get_advice(topic);
                    parts.push(format!("{} {}", intro, advice));
                    parts.push("Does that resonate with your situation?".to_string());
                }
                Intent::AskQuestion(topic) => {
                    let knowledge = self.get_advice(topic); // reuse knowledge base
                    parts.push(format!("Great question. Here's how I'd think about it: {}", knowledge));
                }
                Intent::ShareExperience(topic) => {
                    let acknowledgment = "Thanks for sharing that with me.";
                    let follow_up = self.get_advice(topic);
                    parts.push(format!("{} Based on what you're describing: {}", acknowledgment, follow_up));
                    parts.push("What's your take on that?".to_string());
                }
                Intent::Unknown(text) => {
                    if text.trim().len() > 20 {
                        parts.push("That's interesting — I want to make sure I understand you properly. Could you tell me a bit more?".to_string());
                    } else if text.trim().len() > 5 {
                        parts.push("I'm not sure I caught that. What's on your mind?".to_string());
                    } else {
                        parts.push("Go on — I'm listening.".to_string());
                    }
                }
            }
        }

        if parts.is_empty() {
            return "I'm here. Tell me more.".to_string();
        }

        parts.join(" ")
    }

    fn topic_name(topic: &Topic) -> &'static str {
        match topic {
            Topic::Career => "career & work",
            Topic::MentalHealth => "mental health",
            Topic::Relationships => "relationships",
            Topic::Technology => "technology",
            Topic::Finance => "finance",
            Topic::Learning => "learning & studying",
            Topic::Health => "health",
            Topic::General => "that",
        }
    }

    fn extract_first_word(&self, text: &str) -> String {
        let mut word = text.trim().to_string();
        // Strip punctuation from end
        word = word.trim_end_matches(|c: char| !c.is_alphanumeric()).to_string();
        if let Some(space_idx) = word.find(' ') {
            word = word[..space_idx].to_string();
        }
        if let Some(first) = word.chars().next() {
            let mut cap = first.to_uppercase().to_string();
            cap.push_str(&word[first.len_utf8()..]);
            word = cap;
        }
        word
    }

    pub fn process(&mut self, input: &str) -> String {
        self.context.turn_count += 1;
        let sentiment = self.detect_sentiment(input);
        self.context.sentiment_history.push(sentiment);

        let intents = self.understand_intents(input);

        // Update memory
        for intent in &intents {
            if let Intent::ShareName(name) = intent {
                self.user_name = Some(name.clone());
            }
            // Track last topic for follow-up context
            match intent {
                Intent::AskAdvice(t) | Intent::AskQuestion(t) | Intent::ShareExperience(t) => {
                    self.context.last_topic = Some(t.clone());
                }
                _ => {}
            }
        }

        self.generate_response(intents)
    }
}