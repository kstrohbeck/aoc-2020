use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space0},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::{HashMap, HashSet};

pub fn star_1(data: String) {
    let foods = parse(&data);
    let allergen_map = find_allergens(&foods);
    let allergens = allergen_map.values().copied().collect::<HashSet<&str>>();
    let non_allergen_instances = foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|i| !allergens.contains(*i))
                .count()
        })
        .sum::<usize>();
    println!("{}", non_allergen_instances);
}

pub fn star_2(data: String) {
    let foods = parse(&data);
    let allergen_map = find_allergens(&foods);
    let mut all = allergen_map
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<(&str, &str)>>();
    all.sort_by_key(|(a, _)| *a);
    for (i, ing) in all.iter().map(|(_, i)| i).enumerate() {
        print!("{}", ing);
        if i < ing.len() - 1 {
            print!(",");
        }
    }
    println!("");
}

fn find_allergens<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, &'a str> {
    let mut poss_allergen_map = HashMap::<&str, HashSet<&str>>::new();
    for food in foods {
        let ing_set = food.ingredients.iter().copied().collect::<HashSet<&str>>();
        for allergen in &food.allergens {
            poss_allergen_map
                .entry(allergen)
                .and_modify(|s| {
                    *s = &*s & &ing_set;
                })
                .or_insert(ing_set.clone());
        }
    }

    let mut allergen_map = HashMap::<&str, &str>::new();
    while poss_allergen_map.len() > 0 {
        let (&allergen, ingredients) = poss_allergen_map
            .iter()
            .find(|(_, v)| v.len() == 1)
            .unwrap();
        let ingredient = *ingredients.iter().next().unwrap();
        allergen_map.insert(allergen, ingredient);
        poss_allergen_map.remove(allergen);
        for (_, v) in &mut poss_allergen_map {
            v.remove(ingredient);
        }
    }

    allergen_map
}

fn parse(data: &str) -> Vec<Food> {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| food(s).unwrap().1)
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn food(input: &str) -> IResult<&str, Food> {
    let (input, ingredients) = ingredients(input)?;
    let (input, _) = space0(input)?;
    let (input, allergens) = opt(allergens)(input)?;
    let allergens = allergens.unwrap_or(Vec::new());
    Ok((
        input,
        Food {
            ingredients,
            allergens,
        },
    ))
}

fn allergens(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        tag("(contains"),
        separated_list1(tag(","), preceded(space0, alpha1)),
        tag(")"),
    )(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<&str>> {
    many1(preceded(space0, alpha1))(input)
}
