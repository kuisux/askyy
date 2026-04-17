mod shooting_star;
mod star;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use shooting_star::ShootingStar;
use star::Star;
use std::io::{self, Write};
use std::time::Duration;

fn main() -> io::Result<()> {
    let (width, height) = terminal::size()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let mut rng = rand::thread_rng();

    let mut stars: Vec<Star> = (0..150)
        .map(|_| {
            Star::new(
                rng.gen_range(0..width),
                rng.gen_range(0..height),
                rng.gen_range(0..=255),
            )
        })
        .collect();

    let mut shooting_stars: Vec<ShootingStar> = Vec::new();
    let mut frame_count: u32 = 0;

    loop {
        // --- Input ---
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // --- Spawn shooting star ---
        frame_count += 1;
        if frame_count % 60 == 0 && rng.gen_bool(0.7) {
            shooting_stars.push(ShootingStar::new(&mut rng, width, height));
        }

        // --- Update ---
        for star in &mut stars {
            star.twinkle(&mut rng);
        }

        for shooting_star in &mut shooting_stars {
            shooting_star.update(width, height);
        }

        shooting_stars.retain(|s| s.alive);

        // --- Draw ---
        queue!(stdout, terminal::Clear(ClearType::All))?;

        // Draw background stars
        for star in &stars {
            let color = if star.brightness > 128 {
                Color::White
            } else {
                Color::DarkGrey
            };
            queue!(
                stdout,
                cursor::MoveTo(star.x, star.y),
                SetForegroundColor(color),
                Print(star.character()),
            )?;
        }

        // Draw shooting stars
        for shooting_star in &shooting_stars {
            let trail_len = shooting_star.trail.len();
            for (i, (tx, ty)) in shooting_star.trail.iter().enumerate() {
                let color = if i > trail_len * 3 / 4 {
                    Color::White
                } else if i > trail_len / 2 {
                    Color::Grey
                } else {
                    Color::DarkGrey
                };
                queue!(
                    stdout,
                    cursor::MoveTo(*tx as u16, *ty as u16),
                    SetForegroundColor(color),
                    Print(ShootingStar::trail_char(i, trail_len)),
                )?;
            }

            queue!(
                stdout,
                cursor::MoveTo(shooting_star.x as u16, shooting_star.y as u16),
                SetForegroundColor(Color::White),
                Print('✦'),
            )?;
        }

        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(50));
    }

    // Cleanup
    execute!(stdout, cursor::Show)?;
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
