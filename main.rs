use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s


// This enum stores a different struct for each ship class.
pub enum Ship {
    Fighter(Fighter),
    Missile(Missile),
}

pub enum State {
    Attack,
    Defend,
    Search,
}   

impl Ship {
    pub fn new() -> Ship {
        match class() {
            Class::Fighter => Ship::Fighter(Fighter::new()),
            Class::Missile => Ship::Missile(Missile::new()),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self) {
        match self {
            Ship::Fighter(fighter) => fighter.tick(),
            Ship::Missile(missile) => missile.tick()
        }
    }
}

// Fighters
pub struct Fighter {
    pub move_target: Vec2,
    pub enemy_ang: f64,
    pub radio_channel: usize,
    pub enemy_latest_pos: Vec<Vec2>,
    pub missile_latest_pos: Vec<Vec2>,
    pub state: State,
    pub tick_counter: u32,
}

impl Fighter {
    pub fn new() -> Self {
        Self {
            move_target: vec2(0.0, 0.0),
            enemy_ang: 0.0,
            enemy_latest_pos: Vec::new(),
            radio_channel: (9),
            missile_latest_pos: Vec::new(),
            state: State::Attack,
            tick_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.tick_counter >= (30) {
            if let State::Defend = self.state {
               
            }
            else {
                self.state = State::Search;
                self.tick_counter = 0;
            }
        }

        if self.enemy_latest_pos.len() > 0 {
            let average_position = get_approx_position(&self.enemy_latest_pos);
            // Draw a diamond on the position
            for i in 1000..1010 {
            draw_diamond(average_position, i as f64, 0xf0f0f0);
            }
            draw_line(vec2(average_position[0] - 100000.0, average_position[1]), vec2(average_position[0] + 100000.0, average_position[1]), 0xf0f0f0);
            for i in 1..5 {
                draw_line(vec2(average_position[0] - (3000  - (i * 500)) as f64, average_position[1] - (i * 500) as f64), vec2(average_position[0] + (3000  - (i * 500)) as f64, average_position[1] - (i * 500) as f64), 0xf0f0f0);
            }
            draw_line(vec2(average_position[0], average_position[1] - 100000.0), vec2(average_position[0], average_position[1] + 100000.0), 0xf0f0f0);
        }
        
        // State manager
        match self.state {
            State::Search =>self.search_mode(),
            State::Attack =>self.attack_mode(),
            State::Defend =>self.defend_mode(),
            _=>{},
        }

        self.tick_counter += 1;
    }

    pub fn search_mode(&mut self)
    {
        debug!("Search");
        if self.tick_counter < 4 {
            // Scan the radar around in a circle.
            // seek(self.move_target, vec2(0.0, 0.0), true);

            let average_position = get_approx_position(&self.enemy_latest_pos);
            let dp = average_position - position();
            turn_to(dp.angle());

            if let Some(contact) = scan().filter(|c| c.class == Class::Missile)
            {
                debug!("Contact");
                let average_position = approx_position(&contact, &mut self.missile_latest_pos, 5);
                if position().distance(average_position) < 10000.0 {
                    self.state = State::Defend;
                    self.tick_counter = 0;
                    self.enemy_ang = heading();
                    return;
                }
            } else {
                set_radar_heading(radar_heading() + radar_width());
                set_radar_width(2.0);
            }
        }else {
            self.state = State::Attack;
        }
    }

    pub fn attack_mode(&mut self)
    {
        debug!("Attack");
        set_radio_channel(self.radio_channel);
        if let Some(contact) = scan().filter(|c| c.class == Class::Fighter) {
            let average_position = approx_position(&contact, &mut self.enemy_latest_pos, 10);
            let dp = average_position - position();

            // Point the radar at the target and focus the beam.
            set_radar_heading(dp.angle());
            let dyn_width = radar_calculate_width(position().distance(average_position));
            set_radar_width(dyn_width);

            // Fly towards the target.
            unpredictible_trajectory(average_position, contact.velocity, true);

            // The missile will fly towards this position and acquire the target with radar
            // when close enough.
            send(make_orders(average_position, contact.velocity));
            // Missiles
            if reload_ticks(1) == 0 {
                fire(1);
            }
            turn_to(dp.angle());
        } else {
            // Scan the radar around in a circle.
            set_radar_heading(radar_heading() + radar_width());
            set_radar_width(TAU / 120.0);
            seek(self.move_target, vec2(0.0, 0.0), true);

            let average_position = get_approx_position(&self.enemy_latest_pos);
            let dp = average_position - position();
            turn_to(dp.angle());
        }
    }

    pub fn defend_mode(&mut self)
    {
        debug!("Defend");
        if let Some(contact) = scan().filter(|c| c.class == Class::Missile) {
            let average_position = approx_position(&contact, &mut self.missile_latest_pos, 5);

            // Draw a diamond on the position
            //for i in 0..50 {
            //    draw_diamond(average_position, i as f64, 0xf0f0f0);
            //}

            let dp = average_position - position();

            // Point the radar at the target and focus the beam.
            set_radar_heading(dp.angle());
            let dyn_width = radar_calculate_width(position().distance(average_position));
            set_radar_width(dyn_width);

            // Guns
            if let angle = custom_lead_target(average_position, contact.velocity) {
                //Random jitter makes it more likely to hit accelerating targets.
                let angle = angle + rand(-1.0, 1.0);
                turn_to(angle);
                fire(0);
            }
            turn_to(dp.angle() + rand(-0.53, 0.53));
        } else {
            self.state = State::Search;
        }
    }
}

// Missiles and Torpedos
pub struct Missile {
    target_position: Vec2,
    target_velocity: Vec2,
    radio_channel: usize,
}

impl Missile {
    pub fn new() -> Self {
        let (target_position, target_velocity) = parse_orders(receive());
        Self {
            target_position,
            target_velocity,
            radio_channel: (9),
        }
    }

    pub fn tick(&mut self) {
        debug!("enemy pos: {0:?}", self.target_position);
        debug!("enemy vel: {0:?}", self.target_velocity);
        self.target_position += self.target_velocity * TICK_LENGTH;

        let target_classe = Class::Fighter;
        if let Some(contact) = scan().filter(|c| c.class == target_classe ) {
            // si on voit l'ennemi dans le viseur on lui fonce dessus
            let dp = contact.position - position();
            set_radar_heading(dp.angle());
            let dyn_width = radar_calculate_width(position().distance(contact.position));
            set_radar_width(dyn_width);
            self.target_position = contact.position;
            self.target_velocity = contact.velocity;
        } else {
            // sinnon on regarde si le vaisseau mère nous envoie la position de l'ennemi par radio
            set_radio_channel(self.radio_channel);
            if let Some(msg) = receive() {
                let enemy_pos_x= msg[0];
                let enemy_pos_y= msg[1];
                let enemy_vel_x= msg[2];
                let enemy_vel_y= msg[3];

                // Gestions des erreurs
                let received_target_position = vec2(enemy_pos_x, enemy_pos_y);
                let received_target_velocity = vec2(enemy_vel_x, enemy_vel_y);

                debug!("received pos: {0:?}", received_target_position);
                debug!("received vel: {0:?}", received_target_velocity);

                if received_target_position.x < world_size() && received_target_position.x > -world_size() {
                    self.target_position = received_target_position;
                }
                if received_target_velocity.x < self.target_velocity.x + 1000.0 {
                    self.target_velocity = received_target_velocity;
                }
            }
            set_radar_heading(
                (self.target_position - position()).angle() + rand(-1.0, 1.0) * TAU / 32.0,
            );
            set_radar_width(1.0);
        }
        
        // dans tous les cas on va à la position donnée.
        seek(self.target_position, self.target_velocity, true);
        activate_ability(Ability::Boost);
        // si on est à proximité de l'ennemi on explose
        if position().distance(self.target_position) / velocity().length() < 0.1 {
            explode();
        }   
    }
}

// Library functions
// Approxym the ennemy position
pub fn approx_position(contact: &ScanResult, latest_pos: &mut Vec<Vec2>, max: i32) -> Vec2
{
    if latest_pos.len() >= max as usize {
        for n in 1..(max as usize)  {
            latest_pos[n-1] = latest_pos[n];
        }
        latest_pos.pop();
    }
    latest_pos.push(contact.position);
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let count = latest_pos.len();
    for i in 0..count {
        sum_x += latest_pos[i][0];
        sum_y += latest_pos[i][1];
    }
    let average_position = vec2(sum_x / count as f64, sum_y / count as f64);

    average_position
}

// Approxym the ennemy position
pub fn get_approx_position(latest_pos: &Vec<Vec2>) -> Vec2
{
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let count = latest_pos.len();
    for i in 0..count {
        sum_x += latest_pos[i][0];
        sum_y += latest_pos[i][1];
    }
    let average_position = vec2(sum_x / count as f64, sum_y / count as f64);

    // Draw a diamond on the position
    for i in 0..15 {
        draw_diamond(average_position, i as f64, 0xff0000);
    }
    average_position
}

/// Flies towards a target which has the given position and velocity.
pub fn seek(p: Vec2, v: Vec2, turn: bool) {
    let dp = p - position();
    let dv = v - velocity();
    let low_fuel = fuel() != 0.0 && fuel() < 500.0;

    // Component of dv perpendicular to dp
    let badv = -(dv - dv.dot(dp) * dp.normalize() / dp.length());
    // Acceleration towards the target
    let forward = if low_fuel { vec2(0.0, 0.0) } else { dp };
    let a = (forward - badv * 10.0).normalize() * max_forward_acceleration();
    accelerate(a);

    if turn {
        turn_to(a.angle());
    }
}

/// Flies with an unpredictible trajectory toward the given position and velocity.
pub fn unpredictible_trajectory(p: Vec2, v: Vec2, turn: bool) {
    let distance = position().distance(p);
    let dp = p - position();
    let dv = v - velocity();
    let low_fuel = fuel() != 0.0 && fuel() < 500.0;

    // Component of dv perpendicular to dp
    let badv = -(dv - dv.dot(dp) * dp.normalize() / dp.length());
    // Acceleration towards the target
    let forward = if low_fuel { vec2(0.0, 0.0) } else { dp };
    let a = (forward - badv * 10.0).normalize() * max_forward_acceleration();

    let a_diff = angle_diff(a.angle(), heading());
    
    let pass_dist = 5000.0;
    // let angle = 1.0 / (distance / pass_dist);

    // let real_angle = a_diff + 0.175;
    // let opp = real_angle.tan() * distance;

    let point = position() + vec2(distance * a.angle().cos(), pass_dist * a.angle().sin());
    
    for i in 0..20 {
        draw_diamond(point, i as f64, 0x00ff00);
    }

    seek(point, v, turn);
}

/// Flies with an unpredictible trajectory toward the given position and velocity.
pub fn seek_unpredictible2(p: Vec2, v: Vec2, turn: bool) {
    let distance = position().distance(p);
    let pass_dist = 5000.0;
    let angle = 1.0 / (distance / pass_dist);

    let mut x_opp = 1.0;
    let mut y_opp = 1.0;
    let mut head = radar_heading();
    let mut final_angle: f64;

    if head > PI/2.0 && head < PI {
        head = 180.0 - head;
        final_angle = head;
        x_opp = -1.0;
    }
    else if head > PI && head < 3.0*PI/2.0 {
        head += 180.0;
        final_angle = head - angle;
        x_opp = -1.0;
        y_opp = -1.0;
    }
    else if head > 3.0*PI/2.0 && head < 2.0*PI {
        head = 360.0 - head;
        final_angle = head + angle;
        y_opp = -1.0;
    }
    else {
        final_angle = head - angle;
    }

    let z = distance / angle.cos();
    let x = (final_angle.cos() * z) * x_opp;
    let y = (final_angle.sin() * z) * y_opp;

    let point = position() + vec2(x, y);

    for i in 0..100 {
        draw_diamond(point, i as f64, 0x00ff00);
    }

    seek(point, v, turn);
}

/// Turns towards the given heading.
fn turn_to(target_heading: f64) {
    let heading_error = angle_diff(heading(), target_heading);
    turn(2.0 * heading_error);
}

/// Returns the angle at which to shoot to hit the given target.
fn lead_target(
    target_position: Vec2,
    target_velocity: Vec2,
    bullet_speed: f64,
    ttl: f64,
) -> Option<f64> {
    let dp = target_position - position();
    let dv = target_velocity - velocity();
    let predicted_dp = dp + dv * dp.length() / bullet_speed;
    if predicted_dp.length() / bullet_speed < ttl {
        Some(predicted_dp.angle())
    } else {
        None
    }
}

fn custom_lead_target(target_position: Vec2, target_velocity: Vec2) -> f64 {
    let dp = target_position - position();
    let dv = target_velocity - velocity();
    let mut predicted_dp = dp;
    for _ in 0..3 {
        predicted_dp = dp + dv * predicted_dp.length() / BULLET_SPEED;
    }
    predicted_dp.angle()
}

/// Constructs a radio message from two vectors.
fn make_orders(p: Vec2, v: Vec2) -> Message {
    [p.x, p.y, v.x, v.y]
}

/// Reverse of make_orders.
fn parse_orders(msg: Option<Message>) -> (Vec2, Vec2) {
    if let Some(msg) = msg {
        (vec2(msg[0], msg[1]), vec2(msg[2], msg[3]))
    } else {
        (vec2(0.0, 0.0), vec2(0.0, 0.0))
    }
}

fn radar_calculate_width(distance: f64) -> f64 {
    let avg_distance = 20000.0;
    let linked_width = 1.0;
    let error_margin = 1.25;
    linked_width - ((((distance * error_margin) - avg_distance) / 10.0) * 0.00125)
}
    
/// Save and restore radar registers in order to use a single radar for multiple functions.
pub struct RadarRegs {
    heading: f64,
    width: f64,
    min_distance: f64,
    max_distance: f64,
}

impl RadarRegs {
    fn new() -> Self {
        Self {
            heading: 0.0,
            width: TAU / 120.0,
            min_distance: 0.0,
            max_distance: 1e9,
        }
    }

    fn save(&mut self) {
        self.heading = radar_heading();
        self.width = radar_width();
        self.min_distance = radar_min_distance();
        self.max_distance = radar_max_distance();
    }

    fn restore(&self) {
        set_radar_heading(self.heading);
        set_radar_width(self.width);
        set_radar_min_distance(self.min_distance);
        set_radar_max_distance(self.max_distance);
    }
}