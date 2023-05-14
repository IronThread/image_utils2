use ::{
    image::RgbaImage,
    std::{convert::identity, f64::consts::PI},
};

pub fn circle(radius: usize, color: [u8; 3], l: bool) -> Option<RgbaImage> {
    let fill_vel = (PI / radius as f64).ceil() as usize;

    let mut fill = fill_vel;
    let mut unfill = false;
    let transparent = [255, 255, 255, 0];

    let mut vec: Vec<u8> = Vec::with_capacity(
        (radius.checked_mul(radius)?.checked_mul(4)?).next_power_of_two(),
    );

    let mut vec2 = vec![false; radius];

    let even = radius % 2 == 0;

    let first_center = radius / 2;
    let second_center = first_center + 1;

    if even {
        vec2[first_center] = true;
        vec2[second_center] = true;
    } else {
        vec2[second_center] = true;
    }

    for i in 0..radius {
        if l && i != 0 {
            let mut defade = true;

            for (i, e) in vec2.iter().copied().enumerate() {
               if e {
                    vec.extend(color);

                    if even {
                        if i == first_center || i == second_center {
                            vec.push(255);
                            defade = false;                            
                        } else {
                            if defade {
                                let diff = first_center - i;
                                vec.push(255 / diff as u8);
                            } else {
                                let diff = i - second_center;
                                vec.push(255 / diff as u8);
                            }
                        }
                    } else {
                        if i == second_center {
                            vec.push(255);
                            defade = false;                            
                        } else {
                            if defade {
                                let diff = second_center - i;
                                vec.push(255 / diff as u8);
                            } else {
                                let diff = i - second_center;
                                vec.push(255 / diff as u8);
                            }
                        }
                    }

               } else {
                   vec.extend(transparent);
               }
            }
        } else {
            for e in vec2.iter().copied() {
                if e {
                    vec.extend(color);
                    vec.push(255);
                } else {
                    vec.extend(transparent);
                }
            }
        }

        fill -= 1;

        if fill == 0 {
            fill = fill_vel;
            if unfill {
                decrease_fill(&mut vec2);
            } else {
                if increase_fill(&mut vec2).is_none() {
                    unfill = true;
                }
            }
        }
    }

    let a = radius as u32;
    RgbaImage::from_vec(a, a, vec)
}

fn increase_fill(v: &mut Vec<bool>) -> Option<()> {
    let rpos = v.iter().copied().rposition(identity)?;
    let pos = v.iter().copied().position(identity)?;

    *v.get_mut(pos - 1)? = true;
    *v.get_mut(rpos + 1)? = true;

    Some(())
}

fn decrease_fill(v: &mut Vec<bool>) -> Option<()> {
    let rpos = v.iter().copied().rposition(identity)?;
    let pos = v.iter().copied().position(identity)?;

    *v.get_mut(pos)? = false;
    *v.get_mut(rpos)? = false;

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        circle(500, [255, 0, 0], false).unwrap().save("regular_circle.png").unwrap();
        circle(501, [255, 0, 0], false).unwrap().save("regular_circle_odd.png").unwrap();
        circle(500, [255, 0, 0], true).unwrap().save("light_surrounded_circle.png").unwrap();
        circle(501, [255, 0, 0], true).unwrap().save("light_surrounded_circle_odd.png").unwrap();
    }
}
