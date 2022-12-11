use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashMap;

type Team = String;

#[derive(Deserialize, Debug)]
struct Game {
    conference_game: bool,
    home_team: Team,
    home_points: i32,
    away_team: Team,
    away_points: i32,
}

fn main() {
    let games: Vec<Game> = serde_json::from_slice(include_bytes!("../pac.json")).unwrap();
    let mut scores = HashMap::new();
    let mut wins = HashMap::new();
    for game in &games {
        if game.conference_game {
            let winner = if game.home_points > game.away_points {
                scores
                    .entry((&game.home_team, &game.away_team))
                    .or_insert(Score::Win(game.home_points, game.away_points));
                scores
                    .entry((&game.away_team, &game.home_team))
                    .or_insert(Score::Loss);
                &game.home_team
            } else {
                scores
                    .entry((&game.away_team, &game.home_team))
                    .or_insert(Score::Win(game.away_points, game.home_points));
                scores
                    .entry((&game.home_team, &game.away_team))
                    .or_insert(Score::Loss);
                &game.away_team
            };
            *wins.entry(winner).or_insert(0) += 1;
        }
    }
    let mut team_wins: Vec<_> = wins.into_iter().collect();
    team_wins.sort_by(|tw1, tw2| break_tie(tw1, tw2, &scores));
    println!(
        r#"<html><head><link href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous"></head><body><table class="table table-bordered"><tr><th></th>"#
    );
    for (team, _) in team_wins.iter().rev() {
        println!("<th>{team}</th>");
    }
    println!("</tr>");
    for (team, wins) in &team_wins {
        println!("<tr><th>{team} {wins}</th>");
        for (opponent, _) in team_wins.iter().rev() {
            println!(
                "{}",
                scores
                    .get(&(team, opponent))
                    .map(Score::to_html)
                    .unwrap_or(if team == opponent {
                        String::from("<td class=\"table-secondary\"></td>")
                    } else {
                        String::from("<td></td>")
                    })
            );
        }
        println!("</tr>");
    }
    println!("</table></body></html>");
}

// TODO: implement full tie break logic
fn break_tie(
    (t1, w1): &(&Team, i32),
    (t2, w2): &(&Team, i32),
    scores: &HashMap<(&Team, &Team), Score>,
) -> Ordering {
    if w2 < w1 {
        Ordering::Less
    } else if w2 > w1 {
        Ordering::Greater
    } else if let Some(score) = scores.get(&(t1, t2)) {
        match score {
            Score::Win(_, _) => Ordering::Less,
            Score::Loss => Ordering::Greater,
        }
    } else {
        Ordering::Equal
    }
}

enum Score {
    Win(i32, i32),
    Loss,
}

impl Score {
    fn to_html(&self) -> String {
        match self {
            Score::Win(win_points, lose_points) => format!(
                "<td class=\"table-success\">{}-{}</td>",
                win_points, lose_points
            ),
            Score::Loss => String::from("<td class=\"table-danger\"></td>"),
        }
    }
}
