title() = "Snake!"

settings() = {
    background_color: (1, 1, 0.8, 1),
    reload_interval: 0.25,
    reload_key: 1073741882, // F1
    reset_key: 114, // R
    snake_parts: 100,
    snake_parts_size: 5,
    snake_trail: 10,
    turn_left: 97, // A
    turn_right: 100, // D,
    turn_speed: 5,
    speed: 50,
    focus_speed: 1,
    unfocus_speed: .1,
}

fn init_data(settings: {}) -> {} {
    data := {
        snake_body: init_snake_body(
            parts: settings.snake_parts,
            size: settings.snake_parts_size
        ),
        snake_angle: 1,
        pressing_left: false,
        pressing_right: false,
        focused: true,
    }
    data.next_snake_body := data.snake_body
    return clone(data)
}

fn init_snake_body_parts_size(parts: f64, size: vec4) -> [] {
    body := []
    // end := [(parts - 1) * size, (parts - 1) * size]
    end := (0, 0)
    for i parts {
        push(mut body, end - i * size)
    }
    return clone(body)
}

fn render(settings: {}, data: {}) {
    radius := 20
    offset := 1
    n := len(data.snake_body)
    d := []
    clear(dlist: mut d, color: settings.background_color)
    for i n-1 {
        line(dlist: mut d, color: (.2, .2, 0, 1), radius: 1,
             from: data.snake_body[i+1], to: data.snake_body[i])
    }
    for i n {
        pos := data.snake_body[i]
        circle(dlist: mut d, color: (.2, .2, 0, 1),
               center: pos, radius: radius)
    }
    if n > 0 {
        dir_len := 20
        pos := data.snake_body[0]
        dir := dir(angle: data.snake_angle)
        pos2 := pos + dir * dir_len
        line(dlist: mut d, color: (0, 0, 1, 1), radius: 1, from: pos, to: pos2)
    }

    red := (0, 0.4, 0, 1)
    laser := (1, 0, 0, 1)

    walls := [
        [red, (100, 0), (200, 100)],
        [red, (200, 100), (200, 200)],
        [red, (300, 100), (300, 200)],
        [laser, (300, 200), (400, 200)]
    ]

    for i len(walls) {
        line(dlist: mut d, color: walls[i][0], radius: 5, from: walls[i][1], to: walls[i][2])
    }

    draw(d)
}

fn update(mut data: {}, settings: {}, dt: f64) {
    if data.pressing_left {
        data.snake_angle -= settings.turn_speed * dt
    }
    if data.pressing_right {
        data.snake_angle += settings.turn_speed * dt
    }
    // Update snake body.
    for i len(data.snake_body) {
        pos := data.snake_body[i]
        speed := dt * settings.speed
        dir := if i == 0 {
                dir(angle: data.snake_angle)
            } else {
                prev_pos := data.snake_body[i - 1]
                diff := prev_pos - pos
                len := |diff|
                if len > settings.snake_parts_size {
                    diff / len
                } else {
                    (0, 0)
                }
            }
        data.next_snake_body[i] = pos + dir * speed
    }
    data.snake_body = clone(data.next_snake_body)
}
