fn change_score(amount:u32) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.state.score += amount;
}