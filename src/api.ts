import { invoke } from "@tauri-apps/api/core";

export type ClubRow = { id: number; name: string; short_name: string; nation: string; reputation: number; primary_color: string };
export type CompRow = { id: number; name: string; nation: string; kind: string };
export type NewGameResult = { game_date: string; season: string; clubs: ClubRow[]; competitions: CompRow[] };
export type GameStateRow = { game_date: string; season: string; user_club_id: number | null; user_club_name: string | null };
export type StandingRow = { position: number; club_id: number; club_name: string; short_name: string; played: number; won: number; drawn: number; lost: number; goals_for: number; goals_against: number; goal_difference: number; points: number };
export type FixtureRow = { id: number; round: number; date: string; home_id: number; home_name: string; home_short: string; away_id: number; away_name: string; away_short: string; home_score: number; away_score: number; status: string };
export type PlayerRow = { id: number; first_name: string; last_name: string; common_name: string; age: number; nation: string; position: string; ca: number; pa: number; wage: number; condition: number; morale: number; attrs: { passing:number; finishing:number; dribbling:number; tackling:number; vision:number; anticipation:number; positioning:number; stamina:number; acceleration:number; pace:number; composure:number; technique:number; reflexes:number } };
export type AdvanceResult = { from_date: string; to_date: string; matches_played: number; results: string[] };

export type MatchSnapshot = { state: string; half: number; time_seconds: number; score: [number,number]; fouls: [number,number]; shots: [number,number]; possession: [number,number]; players: { id:number; team_id:number; shirt:number; x:number; y:number; stamina:number; role:string; on_pitch:boolean }[]; ball: [number,number]; ball_holder: number | null; events: { minute:number; second:number; kind:string; team_id:number; player_id:number|null; description:string; x:number; y:number }[] };
export type MarketPlayer = { id:number; name:string; age:number; nation:string; position:string; ca:number; pa:number; club_id:number; club_name:string; club_short:string; value:number; wage:number; contract_end:string };
export type OfferRow = { id:number; player_id:number; player_name:string; from_club:string; from_club_id:number; to_club:string; to_club_id:number; fee:number; status:string; date:string };
export type TrainingRow = { day:number; type_id:number; type_name:string; category:string; intensity:number };
export type ProgressRow = { player_id:number; name:string; position:string; ca:number; pa:number; age:number; improvement:number };
export type FinanceRow = { club_id:number; club_name:string; balance:number; transfer_budget:number; wage_budget:number; total_wages:number; sponsorship:number; ticket_income:number; prize_money:number; weekly_wages:number; monthly_balance:number };
export type InboxRow = { id:number; sender:string; subject:string; body:string; date:string; is_read:number; is_important:number };

export const api = {
  newGame: (userClubId?: number) => invoke<NewGameResult>("new_game", { userClubId }),
  getGameState: () => invoke<GameStateRow>("get_game_state"),
  advanceDay: () => invoke<AdvanceResult>("advance_day_cmd"),
  advanceWeek: () => invoke<AdvanceResult[]>("advance_week_cmd"),
  getStandings: (competitionId: number) => invoke<StandingRow[]>("get_standings", { competitionId }),
  getFixtures: (competitionId: number) => invoke<FixtureRow[]>("get_fixtures", { competitionId }),
  getSquad: (clubId: number) => invoke<PlayerRow[]>("get_squad", { clubId }),
  getCompetitions: () => invoke<CompRow[]>("get_competitions"),
  getNextFixture: (clubId: number) => invoke<FixtureRow | null>("get_next_fixture", { clubId }),
  startLive: (matchId: number) => invoke<MatchSnapshot>("start_live_match", { matchId }),
  tickLive: (ticks?: number) => invoke<MatchSnapshot>("tick_live", { ticks }),
  getLive: () => invoke<MatchSnapshot>("get_live_snapshot"),
  getMarket: () => invoke<MarketPlayer[]>("get_market"),
  getOffers: () => invoke<OfferRow[]>("get_offers"),
  makeOffer: (playerId:number, fee:number) => invoke<string>("make_offer", { playerId, fee }),
  respondOffer: (offerId:number, accept:boolean) => invoke<string>("respond_offer", { offerId, accept }),
  getTrainingSchedule: () => invoke<TrainingRow[]>("get_training_schedule"),
  setTrainingSchedule: (schedule:[number,number,number][]) => invoke<string>("set_training_schedule", { schedule }),
  getTrainingProgress: () => invoke<ProgressRow[]>("get_training_progress"),
  getTrainingTypes: () => invoke<[number,string,string,number][]>("get_training_types"),
  getFinance: () => invoke<FinanceRow>("get_finance"),
  getInjuries: () => invoke<[number,string,string,string,string][]>("get_injuries"),
  getInbox: () => invoke<InboxRow[]>("get_inbox"),
  markRead: (msgId:number) => invoke<void>("mark_read", { msgId }),
  markAllRead: () => invoke<void>("mark_all_read"),
  checkSeasonFinished: () => invoke<boolean>("check_season_finished"),
  rolloverSeason: () => invoke<string>("rollover_season_cmd"),
  ping: () => invoke<string>("ping"),
};
