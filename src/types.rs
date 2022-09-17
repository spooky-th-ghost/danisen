use uuid::{uuid, Uuid};

pub struct User {
    user_id: Uuid,
    player_id: Uuid,
}

pub enum Role {
    User,
    Moderator,
    Admin,
}

pub struct Player {
    pub player_id: Uuid,
    pub display_name: String,
    pub discord_name: String,
    pub rank: Rank,
    pub team1: Option<Team>,
    pub team2: Option<Team>,
    pub team3: Option<Team>,
}

impl Player {
    pub fn can_play(&self, other: Player) -> bool {
        self.rank.can_play(other.rank)
    }
}

pub struct Rank {
    pub name: RankName,
    pub points: i8,
}

impl Rank {
    pub fn from_rankname(name: RankName) -> Self {
        Rank { name, points: 0 }
    }

    pub fn update(&mut self, match_result: MatchResult) {
        let points = match match_result {
            MatchResult::Win => self.points + 1,
            MatchResult::Lose => self.points - 1,
        };

        use RankName::*;
        match self.name {
            Dan1 => {
                if points > 2 {
                    self.name = Dan2;
                    self.points = 0;
                } else if points < 0 {
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Dan2 => {
                if points > 2 {
                    self.name = Dan3;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Dan1;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Dan3 => {
                if points > 2 {
                    self.name = Dan4;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Dan2;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Dan4 => {
                if points > 2 {
                    self.name = Dan5;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Dan3;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Dan5 => {
                if points > 2 {
                    self.name = Dan6;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Dan4;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Dan6 => {
                if points > 2 {
                    self.name = Dan7;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Dan5;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Dan7 => {
                if points > 2 {
                    self.name = Strong;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Dan6;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Strong => {
                if points > 4 {
                    self.name = Valor;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Strong;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Valor => {
                if points > 4 {
                    self.name = Royal;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Strong;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Royal => {
                if points > 4 {
                    self.name = Emperor;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Valor;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Emperor => {
                if points > 4 {
                    self.name = Lord;
                    self.points = 0;
                } else if points < -2 {
                    self.name = Royal;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
            Lord => {
                if points > 4 {
                    self.points = 5;
                } else if points < -2 {
                    self.name = Emperor;
                    self.points = 0;
                } else {
                    self.points = points;
                }
            }
        }
    }

    pub fn can_play(&self, other: Rank) -> bool {
        use RankName::*;
        match &self.name {
            Dan1 => matches!(other.name, Dan1 | Dan2),
            Dan2 => matches!(other.name, Dan1 | Dan2 | Dan3),
            Dan3 => matches!(other.name, Dan2 | Dan3 | Dan4),
            Dan4 => matches!(other.name, Dan3 | Dan4 | Dan5),
            Dan5 => matches!(other.name, Dan4 | Dan5 | Dan6),
            Dan6 => matches!(other.name, Dan5 | Dan6 | Dan7),
            Dan7 => matches!(other.name, Dan6 | Dan7 | Strong),
            Strong => matches!(other.name, Dan7 | Strong | Valor),
            Valor => matches!(other.name, Strong | Valor | Royal),
            Royal => matches!(other.name, Valor | Royal | Emperor),
            Emperor => matches!(other.name, Royal | Emperor | Lord),
            Lord => matches!(other.name, Emperor | Lord),
        }
    }
}

impl Default for Rank {
    fn default() -> Self {
        Rank {
            name: RankName::Dan1,
            points: 0,
        }
    }
}

pub struct PostMatch {
    pub player_1_id: Uuid,
    pub player_1_team: Team,
    pub player_2_id: Uuid,
    pub player_2_team: Team,
    pub winner: Uuid,
}

#[derive(Clone, Copy)]
pub enum MatchResult {
    Win,
    Lose,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RankName {
    Dan1,
    Dan2,
    Dan3,
    Dan4,
    Dan5,
    Dan6,
    Dan7,
    Strong,
    Valor,
    Royal,
    Emperor,
    Lord,
}

pub struct Team {
    pub point: Character,
    pub mid: Option<Character>,
    pub anchor: Option<Character>,
}

pub struct Character {
    pub name: CharacterName,
    pub icon: String,
}

impl Character {
    fn new(name: CharacterName, icon: String) -> Self {
        Character { name, icon }
    }
    fn from_name(name: CharacterName) -> Self {
        use CharacterName::*;
        match name {
            Annie => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/5/5c/Icon-Annie.png/revision/latest/scale-to-width-down/30?cb=20210308142433".to_string()),
            Beowulf => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/b/b4/Icon-Beowulf.png/revision/latest/scale-to-width-down/100?cb=20150418150623".to_string()),
            BigBand => Self::new(name, "https://static.wikia.nocookie.net/skullgirls/images/3/37/Icon-Big_Band.png/revision/latest/scale-to-width-down/100?cb=20150418152045".to_string()),
            BlackDhalia => Self::new(name, "".to_string()),
            Cerabella => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/9/90/Icon-Cerebella.png/revision/latest/scale-to-width-down/100?cb=20150418151607".to_string()),
            Double => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/a/a5/Icon-Double.png/revision/latest/scale-to-width-down/100?cb=20150418151949".to_string()),
            Eliza => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/b/b6/Icon-Eliza.png/revision/latest/scale-to-width-down/100?cb=20150418152202".to_string()),
            Fillia => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/1/1d/Icon-Filia.png/revision/latest/scale-to-width-down/40?cb=20150418151218".to_string()),
            Fukua => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/3/37/Icon-Fukua.png/revision/latest/scale-to-width-down/100?cb=20150418152407".to_string()),
            Marie => Self::new(name,"".to_string()),
            MsFortune => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/6/62/Icon-Ms._Fortune.png/revision/latest/scale-to-width-down/100?cb=20150418151755".to_string()),
            Painwheel => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/a/ab/Icon-Painwheel.png/revision/latest/scale-to-width-down/100?cb=20150418151910".to_string()),
            Parasoul => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/c/c8/Icon-Parasoul.png/revision/latest/scale-to-width-down/100?cb=20150418151839".to_string()),
            Peacock => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/a/ac/Icon-Peacock.png/revision/latest/scale-to-width-down/100?cb=20150418151750".to_string()),
            RoboFortune => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/a/a3/Icon-Robo-Fortune.png/revision/latest/scale-to-width-down/100?cb=20150423041704".to_string()),
            Squiggly => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/2/24/Icon-Squigly.png/revision/latest/scale-to-width-down/100?cb=20150418152035".to_string()),
            Umbrella => Self::new(name,"https://wiki.gbl.gg/images/0/08/SG_umb_icon.png".to_string()),
            Valentine => Self::new(name,"https://static.wikia.nocookie.net/skullgirls/images/1/13/Icon-Valentine.png/revision/latest/scale-to-width-down/100?cb=20150418151931".to_string()),
        }
    }
}

pub enum CharacterName {
    Annie,
    Beowulf,
    BigBand,
    BlackDhalia,
    Cerabella,
    Double,
    Eliza,
    Fillia,
    Fukua,
    Marie,
    MsFortune,
    Painwheel,
    Parasoul,
    Peacock,
    RoboFortune,
    Squiggly,
    Umbrella,
    Valentine,
}
