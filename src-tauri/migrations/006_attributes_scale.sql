UPDATE player_attributes SET
  first_touch=MIN(100, first_touch*5), dribbling=MIN(100, dribbling*5), ball_control=MIN(100, ball_control*5), technique=MIN(100, technique*5),
  passing=MIN(100, passing*5), vision=MIN(100, vision*5), crossing=MIN(100, crossing*5), long_shots=MIN(100, long_shots*5),
  finishing=MIN(100, finishing*5), heading=MIN(100, heading*5), penalty_taking=MIN(100, penalty_taking*5),
  tackling=MIN(100, tackling*5), marking=MIN(100, marking*5), interception=MIN(100, interception*5), blocking=MIN(100, blocking*5),
  anticipation=MIN(100, anticipation*5), decisions=MIN(100, decisions*5), positioning=MIN(100, positioning*5), off_the_ball=MIN(100, off_the_ball*5), work_rate=MIN(100, work_rate*5),
  composure=MIN(100, composure*5), concentration=MIN(100, concentration*5), determination=MIN(100, determination*5), bravery=MIN(100, bravery*5),
  aggression=MIN(100, aggression*5), leadership=MIN(100, leadership*5), teamwork=MIN(100, teamwork*5), flair=MIN(100, flair*5),
  acceleration=MIN(100, acceleration*5), pace=MIN(100, pace*5), agility=MIN(100, agility*5), balance=MIN(100, balance*5), stamina=MIN(100, stamina*5),
  strength=MIN(100, strength*5), jumping=MIN(100, jumping*5),
  reflexes=MIN(100, reflexes*5), handling=MIN(100, handling*5), one_on_ones=MIN(100, one_on_ones*5), positioning_gk=MIN(100, positioning_gk*5),
  rushing_out=MIN(100, rushing_out*5), throwing=MIN(100, throwing*5), kicking=MIN(100, kicking*5),
  professionalism=MIN(100, professionalism*5), consistency=MIN(100, consistency*5), important_matches=MIN(100, important_matches*5), injury_proneness=MIN(100, injury_proneness*5);
