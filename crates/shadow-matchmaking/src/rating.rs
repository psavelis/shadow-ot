//! Rating System Module
//!
//! Implements ELO-based rating calculations for competitive play.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{ArenaMatch, MatchResult, MatchType};

/// Player rating information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRating {
    /// Character ID
    pub character_id: Uuid,
    /// Current rating
    pub rating: i32,
    /// Peak rating achieved
    pub peak_rating: i32,
    /// Games played
    pub games_played: u32,
    /// Games won
    pub games_won: u32,
    /// Games lost
    pub games_lost: u32,
    /// Current win streak
    pub win_streak: u32,
    /// Best win streak
    pub best_win_streak: u32,
    /// Current lose streak
    pub lose_streak: u32,
    /// Rating for specific match types
    pub type_ratings: HashMap<MatchType, i32>,
}

impl PlayerRating {
    /// Create a new player rating (starting at 1000)
    pub fn new(character_id: Uuid) -> Self {
        Self {
            character_id,
            rating: 1000,
            peak_rating: 1000,
            games_played: 0,
            games_won: 0,
            games_lost: 0,
            win_streak: 0,
            best_win_streak: 0,
            lose_streak: 0,
            type_ratings: HashMap::new(),
        }
    }

    /// Get win rate as percentage
    pub fn win_rate(&self) -> f64 {
        if self.games_played == 0 {
            return 0.0;
        }
        (self.games_won as f64 / self.games_played as f64) * 100.0
    }

    /// Record a win
    pub fn record_win(&mut self, rating_change: i32) {
        self.rating += rating_change;
        self.games_played += 1;
        self.games_won += 1;
        self.win_streak += 1;
        self.lose_streak = 0;

        if self.win_streak > self.best_win_streak {
            self.best_win_streak = self.win_streak;
        }

        if self.rating > self.peak_rating {
            self.peak_rating = self.rating;
        }
    }

    /// Record a loss
    pub fn record_loss(&mut self, rating_change: i32) {
        self.rating = (self.rating - rating_change).max(0);
        self.games_played += 1;
        self.games_lost += 1;
        self.lose_streak += 1;
        self.win_streak = 0;
    }

    /// Record a draw
    pub fn record_draw(&mut self) {
        self.games_played += 1;
        self.win_streak = 0;
        self.lose_streak = 0;
    }
}

/// Rating system using modified ELO
pub struct RatingSystem {
    /// Player ratings
    ratings: HashMap<Uuid, PlayerRating>,
    /// Base K-factor for rating calculations
    k_factor: f64,
    /// New player K-factor boost
    new_player_boost: f64,
    /// Streak bonus multiplier
    streak_bonus: f64,
}

impl RatingSystem {
    /// Create a new rating system
    pub fn new() -> Self {
        Self {
            ratings: HashMap::new(),
            k_factor: 32.0,
            new_player_boost: 1.5,
            streak_bonus: 0.1,
        }
    }

    /// Get or create a player's rating
    pub fn get_rating(&self, character_id: Uuid) -> PlayerRating {
        self.ratings.get(&character_id)
            .cloned()
            .unwrap_or_else(|| PlayerRating::new(character_id))
    }

    /// Get mutable rating
    pub fn get_rating_mut(&mut self, character_id: Uuid) -> &mut PlayerRating {
        self.ratings.entry(character_id)
            .or_insert_with(|| PlayerRating::new(character_id))
    }

    /// Calculate expected score
    fn expected_score(rating_a: i32, rating_b: i32) -> f64 {
        1.0 / (1.0 + 10.0_f64.powf((rating_b - rating_a) as f64 / 400.0))
    }

    /// Calculate K-factor for a player
    fn calculate_k_factor(&self, player: &PlayerRating) -> f64 {
        let mut k = self.k_factor;

        // Boost for new players (under 30 games)
        if player.games_played < 30 {
            k *= self.new_player_boost;
        }

        // Reduce K-factor for high-rated players
        if player.rating > 2400 {
            k *= 0.75;
        }

        // Streak bonus
        if player.win_streak >= 3 {
            k *= 1.0 + (self.streak_bonus * player.win_streak.min(5) as f64);
        }

        k
    }

    /// Calculate rating change for a match
    fn calculate_rating_change(
        &self,
        winner_rating: i32,
        loser_rating: i32,
        winner_games: u32,
        loser_games: u32,
    ) -> (i32, i32) {
        let expected_winner = Self::expected_score(winner_rating, loser_rating);
        let expected_loser = 1.0 - expected_winner;

        // Create temporary rating objects for K-factor calculation
        let winner_player = PlayerRating {
            rating: winner_rating,
            games_played: winner_games,
            ..PlayerRating::new(Uuid::nil())
        };
        let loser_player = PlayerRating {
            rating: loser_rating,
            games_played: loser_games,
            ..PlayerRating::new(Uuid::nil())
        };

        let k_winner = self.calculate_k_factor(&winner_player);
        let k_loser = self.calculate_k_factor(&loser_player);

        let winner_change = (k_winner * (1.0 - expected_winner)).round() as i32;
        let loser_change = (k_loser * expected_loser).round() as i32;

        (winner_change.max(1), loser_change.max(1))
    }

    /// Process a completed match
    pub fn process_match(
        &mut self,
        arena_match: &ArenaMatch,
        result: MatchResult,
    ) -> Vec<(Uuid, i32)> {
        let mut changes = Vec::new();

        match result {
            MatchResult::Team1Win | MatchResult::Team2Win => {
                let winning_team = if result == MatchResult::Team1Win { 0 } else { 1 };
                
                // Calculate average ratings per team
                let team1: Vec<_> = arena_match.participants.iter()
                    .filter(|p| p.team == 0)
                    .collect();
                let team2: Vec<_> = arena_match.participants.iter()
                    .filter(|p| p.team == 1)
                    .collect();

                let team1_avg = team1.iter()
                    .map(|p| p.rating_before)
                    .sum::<i32>() / team1.len().max(1) as i32;
                let team2_avg = team2.iter()
                    .map(|p| p.rating_before)
                    .sum::<i32>() / team2.len().max(1) as i32;

                let (winner_avg, loser_avg) = if winning_team == 0 {
                    (team1_avg, team2_avg)
                } else {
                    (team2_avg, team1_avg)
                };

                // Calculate base rating change
                let (gain, loss) = self.calculate_rating_change(
                    winner_avg,
                    loser_avg,
                    10, // Using placeholder games count
                    10,
                );

                // Apply rating changes
                for participant in &arena_match.participants {
                    let rating = self.get_rating_mut(participant.character_id);
                    
                    if participant.team == winning_team {
                        // Bonus for good KDA
                        let kda_bonus = ((participant.stats.kda() - 1.0) * 2.0).max(0.0) as i32;
                        let total_gain = gain + kda_bonus;
                        
                        // Penalty for leaving early
                        let final_gain = if participant.left_early {
                            0 // No gain for quitters
                        } else {
                            total_gain
                        };

                        rating.record_win(final_gain);
                        changes.push((participant.character_id, final_gain));
                    } else {
                        // Reduced loss for good KDA
                        let kda_reduction = ((participant.stats.kda() - 0.5) * 2.0).max(0.0).min(loss as f64 * 0.5) as i32;
                        let final_loss = loss - kda_reduction;
                        
                        // Extra penalty for leaving early
                        let adjusted_loss = if participant.left_early {
                            final_loss * 2
                        } else {
                            final_loss
                        };

                        rating.record_loss(adjusted_loss);
                        changes.push((participant.character_id, -adjusted_loss));
                    }
                }
            }
            MatchResult::Draw => {
                // No rating change for draws
                for participant in &arena_match.participants {
                    let rating = self.get_rating_mut(participant.character_id);
                    rating.record_draw();
                    changes.push((participant.character_id, 0));
                }
            }
            MatchResult::Cancelled | MatchResult::InProgress => {
                // No changes for cancelled/in-progress matches
            }
        }

        changes
    }

    /// Get leaderboard for a match type
    pub fn get_leaderboard(&self, _match_type: MatchType, limit: usize) -> Vec<(Uuid, i32)> {
        let mut players: Vec<_> = self.ratings.iter()
            .map(|(id, r)| (*id, r.rating))
            .collect();

        players.sort_by(|a, b| b.1.cmp(&a.1));
        players.truncate(limit);
        players
    }

    /// Get global leaderboard
    pub fn get_global_leaderboard(&self, limit: usize) -> Vec<(Uuid, i32)> {
        self.get_leaderboard(MatchType::Duel, limit)
    }
}

impl Default for RatingSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_score() {
        // Equal rating should give 0.5 expected
        let expected = RatingSystem::expected_score(1000, 1000);
        assert!((expected - 0.5).abs() < 0.01);

        // Higher rating should have higher expected score
        let higher = RatingSystem::expected_score(1200, 1000);
        assert!(higher > 0.5);
    }

    #[test]
    fn test_rating_change() {
        let system = RatingSystem::new();
        let (gain, loss) = system.calculate_rating_change(1000, 1000, 50, 50);
        
        // Roughly equal change for equal ratings
        assert!((gain - loss).abs() < 5);
    }

    #[test]
    fn test_win_streak() {
        let mut rating = PlayerRating::new(Uuid::new_v4());
        
        rating.record_win(20);
        assert_eq!(rating.win_streak, 1);
        
        rating.record_win(20);
        assert_eq!(rating.win_streak, 2);
        
        rating.record_loss(15);
        assert_eq!(rating.win_streak, 0);
        assert_eq!(rating.lose_streak, 1);
    }
}
