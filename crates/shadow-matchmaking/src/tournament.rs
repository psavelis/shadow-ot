//! Tournament Module
//!
//! Handles tournament creation, brackets, and progression.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{MatchType, MatchResult, Rank};

/// Tournament format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TournamentFormat {
    /// Single elimination bracket
    SingleElimination,
    /// Double elimination bracket
    DoubleElimination,
    /// Round robin
    RoundRobin,
    /// Swiss system
    Swiss,
}

/// Tournament status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TournamentStatus {
    /// Registration open
    Registration,
    /// Registration closed, waiting to start
    Pending,
    /// Tournament in progress
    InProgress,
    /// Tournament complete
    Completed,
    /// Tournament cancelled
    Cancelled,
}

/// A tournament participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentParticipant {
    /// Character ID
    pub character_id: Uuid,
    /// Character name
    pub character_name: String,
    /// Seed number
    pub seed: u32,
    /// Current bracket position
    pub bracket_position: Option<u32>,
    /// Is eliminated
    pub eliminated: bool,
    /// Wins in tournament
    pub wins: u32,
    /// Losses in tournament
    pub losses: u32,
    /// Rating at time of registration
    pub registration_rating: i32,
}

/// A bracket match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketMatch {
    /// Match ID
    pub id: Uuid,
    /// Round number (1 = first round)
    pub round: u32,
    /// Match number within round
    pub match_number: u32,
    /// Participant 1 ID
    pub participant1: Option<Uuid>,
    /// Participant 2 ID
    pub participant2: Option<Uuid>,
    /// Winner ID
    pub winner: Option<Uuid>,
    /// Match result
    pub result: Option<MatchResult>,
    /// Scheduled time
    pub scheduled_at: Option<DateTime<Utc>>,
    /// Completed time
    pub completed_at: Option<DateTime<Utc>>,
    /// Is a bye match
    pub is_bye: bool,
}

impl BracketMatch {
    /// Create a new bracket match
    pub fn new(round: u32, match_number: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            round,
            match_number,
            participant1: None,
            participant2: None,
            winner: None,
            result: None,
            scheduled_at: None,
            completed_at: None,
            is_bye: false,
        }
    }

    /// Check if match is ready to play
    pub fn is_ready(&self) -> bool {
        self.participant1.is_some() && 
        (self.participant2.is_some() || self.is_bye) &&
        self.winner.is_none()
    }

    /// Check if match is complete
    pub fn is_complete(&self) -> bool {
        self.winner.is_some()
    }
}

/// Tournament prizes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TournamentPrize {
    /// Gold amount
    pub gold: u64,
    /// Premium currency
    pub premium_currency: u32,
    /// Items (item_id, count)
    pub items: Vec<(u32, u32)>,
    /// Title granted
    pub title: Option<String>,
    /// Rating bonus
    pub rating_bonus: i32,
}

/// A tournament
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    /// Tournament ID
    pub id: Uuid,
    /// Tournament name
    pub name: String,
    /// Description
    pub description: String,
    /// Match type
    pub match_type: MatchType,
    /// Format
    pub format: TournamentFormat,
    /// Status
    pub status: TournamentStatus,
    /// Maximum participants
    pub max_participants: u32,
    /// Minimum level to enter
    pub min_level: u32,
    /// Minimum rank to enter
    pub min_rank: Option<Rank>,
    /// Entry fee (gold)
    pub entry_fee: u64,
    /// Registration start
    pub registration_start: DateTime<Utc>,
    /// Registration end
    pub registration_end: DateTime<Utc>,
    /// Tournament start
    pub start_time: DateTime<Utc>,
    /// Participants
    pub participants: HashMap<Uuid, TournamentParticipant>,
    /// Bracket matches
    pub bracket: Vec<BracketMatch>,
    /// Current round
    pub current_round: u32,
    /// Prizes by placement
    pub prizes: HashMap<u32, TournamentPrize>,
    /// Created by
    pub created_by: Uuid,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl Tournament {
    /// Create a new tournament
    pub fn new(
        name: &str,
        match_type: MatchType,
        format: TournamentFormat,
        max_participants: u32,
        start_time: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            match_type,
            format,
            status: TournamentStatus::Registration,
            max_participants,
            min_level: 50,
            min_rank: None,
            entry_fee: 0,
            registration_start: now,
            registration_end: start_time - Duration::hours(1),
            start_time,
            participants: HashMap::new(),
            bracket: Vec::new(),
            current_round: 0,
            prizes: HashMap::new(),
            created_by: Uuid::nil(),
            created_at: now,
        }
    }

    /// Register a participant
    pub fn register(
        &mut self,
        character_id: Uuid,
        character_name: &str,
        rating: i32,
    ) -> Result<(), &'static str> {
        if self.status != TournamentStatus::Registration {
            return Err("Registration is not open");
        }

        if self.participants.len() >= self.max_participants as usize {
            return Err("Tournament is full");
        }

        if self.participants.contains_key(&character_id) {
            return Err("Already registered");
        }

        let participant = TournamentParticipant {
            character_id,
            character_name: character_name.to_string(),
            seed: 0,
            bracket_position: None,
            eliminated: false,
            wins: 0,
            losses: 0,
            registration_rating: rating,
        };

        self.participants.insert(character_id, participant);
        Ok(())
    }

    /// Unregister a participant
    pub fn unregister(&mut self, character_id: Uuid) -> Result<(), &'static str> {
        if self.status != TournamentStatus::Registration {
            return Err("Cannot unregister after registration closes");
        }

        self.participants.remove(&character_id)
            .map(|_| ())
            .ok_or("Not registered")
    }

    /// Generate bracket based on seeding
    pub fn generate_bracket(&mut self) {
        // Seed players by rating
        let mut seeded: Vec<_> = self.participants.values_mut().collect();
        seeded.sort_by(|a, b| b.registration_rating.cmp(&a.registration_rating));

        for (i, participant) in seeded.iter_mut().enumerate() {
            participant.seed = (i + 1) as u32;
        }

        // Generate bracket matches based on format
        match self.format {
            TournamentFormat::SingleElimination => {
                self.generate_single_elimination_bracket();
            }
            TournamentFormat::DoubleElimination => {
                self.generate_double_elimination_bracket();
            }
            TournamentFormat::RoundRobin => {
                self.generate_round_robin_bracket();
            }
            TournamentFormat::Swiss => {
                // Swiss is generated round by round
                self.generate_swiss_round();
            }
        }

        self.current_round = 1;
    }

    /// Generate single elimination bracket
    fn generate_single_elimination_bracket(&mut self) {
        let count = self.participants.len();
        let rounds = (count as f64).log2().ceil() as u32;
        
        // Find next power of 2
        let bracket_size = 2_u32.pow(rounds);
        
        let mut seeded: Vec<_> = self.participants.values()
            .map(|p| p.character_id)
            .collect();
        seeded.sort_by_key(|id| self.participants[id].seed);

        // First round matches
        let first_round_matches = bracket_size / 2;
        for i in 0..first_round_matches {
            let mut match_ = BracketMatch::new(1, i + 1);
            
            let p1_idx = i as usize;
            let p2_idx = (bracket_size - 1 - i) as usize;

            if p1_idx < seeded.len() {
                match_.participant1 = Some(seeded[p1_idx]);
            }
            if p2_idx < seeded.len() {
                match_.participant2 = Some(seeded[p2_idx]);
            }

            // Handle byes
            if match_.participant1.is_some() && match_.participant2.is_none() {
                match_.is_bye = true;
                match_.winner = match_.participant1;
            }

            self.bracket.push(match_);
        }

        // Generate subsequent rounds (empty for now)
        let mut matches_in_round = first_round_matches / 2;
        for round in 2..=rounds {
            for i in 0..matches_in_round {
                let match_ = BracketMatch::new(round, i + 1);
                self.bracket.push(match_);
            }
            matches_in_round /= 2;
        }
    }

    /// Generate double elimination bracket
    fn generate_double_elimination_bracket(&mut self) {
        // Start with winner's bracket
        self.generate_single_elimination_bracket();
        
        // Loser's bracket would be generated as matches complete
        // This is a simplified implementation
    }

    /// Generate round robin bracket
    fn generate_round_robin_bracket(&mut self) {
        let participants: Vec<_> = self.participants.keys().cloned().collect();
        let count = participants.len();
        
        let mut match_num = 1;
        for round in 1..=count as u32 - 1 {
            for i in 0..count / 2 {
                let p1_idx = i;
                let p2_idx = count - 1 - i;
                
                if p1_idx != p2_idx {
                    let mut match_ = BracketMatch::new(round, match_num);
                    match_.participant1 = Some(participants[p1_idx]);
                    match_.participant2 = Some(participants[p2_idx]);
                    self.bracket.push(match_);
                    match_num += 1;
                }
            }
        }
    }

    /// Generate next Swiss round
    fn generate_swiss_round(&mut self) {
        // Pair players with similar records
        let mut standings: Vec<_> = self.participants.values()
            .filter(|p| !p.eliminated)
            .collect();
        
        standings.sort_by(|a, b| {
            let a_score = a.wins as i32 - a.losses as i32;
            let b_score = b.wins as i32 - b.losses as i32;
            b_score.cmp(&a_score)
        });

        let round = self.current_round + 1;
        let mut match_num = 1;

        for chunk in standings.chunks(2) {
            if chunk.len() == 2 {
                let mut match_ = BracketMatch::new(round, match_num);
                match_.participant1 = Some(chunk[0].character_id);
                match_.participant2 = Some(chunk[1].character_id);
                self.bracket.push(match_);
                match_num += 1;
            }
        }

        self.current_round = round;
    }

    /// Report a match result
    pub fn report_match_result(
        &mut self,
        match_id: Uuid,
        winner_id: Uuid,
    ) -> Result<(), &'static str> {
        // Extract match info first to avoid borrow conflicts
        let (round, match_number, loser_id) = {
            let match_result = self.bracket.iter_mut()
                .find(|m| m.id == match_id)
                .ok_or("Match not found")?;

            if match_result.winner.is_some() {
                return Err("Match already has a result");
            }

            // Verify winner is a participant
            if match_result.participant1 != Some(winner_id) &&
               match_result.participant2 != Some(winner_id) {
                return Err("Winner is not a participant in this match");
            }

            match_result.winner = Some(winner_id);
            match_result.completed_at = Some(Utc::now());

            // Determine loser
            let loser_id = if match_result.participant1 == Some(winner_id) {
                match_result.participant2
            } else {
                match_result.participant1
            };

            (match_result.round, match_result.match_number, loser_id)
        };

        // Update participant records
        if let Some(winner) = self.participants.get_mut(&winner_id) {
            winner.wins += 1;
        }

        if let Some(loser_id) = loser_id {
            if let Some(loser) = self.participants.get_mut(&loser_id) {
                loser.losses += 1;

                // Eliminate in single elimination
                if self.format == TournamentFormat::SingleElimination {
                    loser.eliminated = true;
                }
            }
        }

        // Advance winner to next round
        self.advance_winner(round, match_number, winner_id);

        Ok(())
    }

    /// Advance winner to next round
    fn advance_winner(&mut self, current_round: u32, match_number: u32, winner_id: Uuid) {
        let next_round = current_round + 1;
        let next_match_number = (match_number + 1) / 2;

        if let Some(next_match) = self.bracket.iter_mut()
            .find(|m| m.round == next_round && m.match_number == next_match_number)
        {
            if match_number % 2 == 1 {
                next_match.participant1 = Some(winner_id);
            } else {
                next_match.participant2 = Some(winner_id);
            }
        }
    }

    /// Check if tournament is complete
    pub fn is_complete(&self) -> bool {
        match self.format {
            TournamentFormat::SingleElimination => {
                // Complete when only one participant not eliminated
                self.participants.values()
                    .filter(|p| !p.eliminated)
                    .count() == 1
            }
            _ => {
                // All matches complete
                self.bracket.iter().all(|m| m.is_complete())
            }
        }
    }

    /// Get current standings
    pub fn get_standings(&self) -> Vec<&TournamentParticipant> {
        let mut standings: Vec<_> = self.participants.values().collect();
        
        standings.sort_by(|a, b| {
            // Sort by wins desc, then losses asc
            if a.wins != b.wins {
                b.wins.cmp(&a.wins)
            } else {
                a.losses.cmp(&b.losses)
            }
        });

        standings
    }

    /// Get the winner
    pub fn get_winner(&self) -> Option<&TournamentParticipant> {
        if !self.is_complete() {
            return None;
        }

        self.participants.values()
            .filter(|p| !p.eliminated)
            .max_by_key(|p| p.wins)
    }
}

/// Tournament manager
pub struct TournamentManager {
    /// Active tournaments
    tournaments: HashMap<Uuid, Tournament>,
    /// Completed tournaments
    completed: Vec<Uuid>,
}

impl TournamentManager {
    /// Create a new tournament manager
    pub fn new() -> Self {
        Self {
            tournaments: HashMap::new(),
            completed: Vec::new(),
        }
    }

    /// Create a new tournament
    pub fn create_tournament(&mut self, tournament: Tournament) -> Uuid {
        let id = tournament.id;
        self.tournaments.insert(id, tournament);
        id
    }

    /// Get a tournament
    pub fn get_tournament(&self, id: Uuid) -> Option<&Tournament> {
        self.tournaments.get(&id)
    }

    /// Get mutable tournament
    pub fn get_tournament_mut(&mut self, id: Uuid) -> Option<&mut Tournament> {
        self.tournaments.get_mut(&id)
    }

    /// Start a tournament
    pub fn start_tournament(&mut self, id: Uuid) -> Result<(), &'static str> {
        let tournament = self.tournaments.get_mut(&id)
            .ok_or("Tournament not found")?;

        if tournament.status != TournamentStatus::Registration {
            return Err("Tournament cannot be started");
        }

        if tournament.participants.len() < 2 {
            return Err("Not enough participants");
        }

        tournament.generate_bracket();
        tournament.status = TournamentStatus::InProgress;

        Ok(())
    }

    /// Get active tournaments
    pub fn get_active_tournaments(&self) -> Vec<&Tournament> {
        self.tournaments.values()
            .filter(|t| t.status == TournamentStatus::InProgress)
            .collect()
    }

    /// Get open registration tournaments
    pub fn get_open_tournaments(&self) -> Vec<&Tournament> {
        self.tournaments.values()
            .filter(|t| t.status == TournamentStatus::Registration)
            .collect()
    }

    /// Process tournaments (check for start times, etc.)
    pub fn process(&mut self) {
        let now = Utc::now();
        
        for tournament in self.tournaments.values_mut() {
            // Auto-close registration
            if tournament.status == TournamentStatus::Registration &&
               now >= tournament.registration_end {
                tournament.status = TournamentStatus::Pending;
            }

            // Auto-start tournament
            if tournament.status == TournamentStatus::Pending &&
               now >= tournament.start_time {
                if tournament.participants.len() >= 2 {
                    tournament.generate_bracket();
                    tournament.status = TournamentStatus::InProgress;
                } else {
                    tournament.status = TournamentStatus::Cancelled;
                }
            }

            // Check for completion
            if tournament.status == TournamentStatus::InProgress &&
               tournament.is_complete() {
                tournament.status = TournamentStatus::Completed;
            }
        }
    }
}

impl Default for TournamentManager {
    fn default() -> Self {
        Self::new()
    }
}
