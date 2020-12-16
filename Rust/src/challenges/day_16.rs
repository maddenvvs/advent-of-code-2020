use super::challenge::{Challenge, Error};
use std::str::FromStr;

type Ticket = Vec<i32>;

struct Rule {
    name: String,
    intervals: Vec<(i32, i32)>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let name = parts.next().unwrap().to_string();
        let intevals_text = parts.next().unwrap().split(" or ");

        let intervals: Vec<(i32, i32)> = intevals_text
            .map(|el| {
                let mut parts = el.split('-').map(|el| el.parse::<i32>().unwrap());

                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();

        Ok(Rule { name, intervals })
    }
}

impl Rule {
    fn apply_to(&self, value: i32) -> bool {
        self.intervals
            .iter()
            .any(|(f, t)| *f <= value && value <= *t)
    }
}

struct Notes {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Notes {
    fn find_error_rate(&self) -> i64 {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.iter())
            .filter(|&&num| self.rules.iter().all(|r| !r.apply_to(num)))
            .map(|&el| el as i64)
            .sum()
    }

    fn find_product_of_departure_fields(&self) -> i64 {
        self.find_field_order()
            .iter()
            .zip(self.my_ticket.iter())
            .filter(|(name, _)| name.starts_with("departure"))
            .map(|(_, &v)| v as i64)
            .product()
    }

    fn find_field_order(&self) -> Vec<&String> {
        let valid_tickets = self.find_valid_tickets();
        let mut possible_fields: Vec<Vec<&String>> = vec![];

        for column in 0..self.my_ticket.len() {
            possible_fields.push(vec![]);

            for rule in self.rules.iter() {
                let mut acceptable_field = true;
                for ticket in valid_tickets.iter() {
                    if !rule.apply_to(ticket[column]) {
                        acceptable_field = false;
                        break;
                    }
                }

                if acceptable_field {
                    possible_fields[column].push(&rule.name);
                }
            }
        }

        let mut solution = vec![];
        Notes::find_rows_order_rec(0, &possible_fields, &mut solution);

        solution
    }

    fn find_rows_order_rec<'a>(
        column: usize,
        possible_fields: &Vec<Vec<&'a String>>,
        temp_solution: &mut Vec<&'a String>,
    ) -> bool {
        if column >= possible_fields.len() {
            return true;
        }

        for name in possible_fields[column].iter() {
            if temp_solution.contains(name) {
                continue;
            }

            temp_solution.push(name);
            if Notes::find_rows_order_rec(column + 1, possible_fields, temp_solution) {
                return true;
            }
            temp_solution.pop();
        }

        return false;
    }

    fn find_valid_tickets(&self) -> Vec<&Ticket> {
        let mut valid_tickets: Vec<&Ticket> = self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.is_ticket_valid(ticket))
            .collect();

        valid_tickets.push(&self.my_ticket);

        valid_tickets
    }

    fn is_ticket_valid(&self, ticket: &Ticket) -> bool {
        ticket
            .iter()
            .all(|&num| self.rules.iter().any(|r| r.apply_to(num)))
    }

    fn parse_rules(rules_text: &str) -> Vec<Rule> {
        rules_text.lines().map(|el| el.parse().unwrap()).collect()
    }

    fn parse_my_ticket(my_ticket_text: &str) -> Ticket {
        Notes::parse_ticket(my_ticket_text.trim_start_matches("your ticket:\n"))
    }

    fn parse_nearby_tickets(nearby_tickets_text: &str) -> Vec<Ticket> {
        nearby_tickets_text
            .trim_start_matches("nearby tickets:\n")
            .lines()
            .map(Notes::parse_ticket)
            .collect()
    }

    fn parse_ticket(ticket_text: &str) -> Ticket {
        ticket_text
            .split(',')
            .map(|el| el.parse().unwrap())
            .collect()
    }
}

impl FromStr for Notes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let rules = Notes::parse_rules(parts.next().unwrap());
        let my_ticket = Notes::parse_my_ticket(parts.next().unwrap());
        let nearby_tickets = Notes::parse_nearby_tickets(parts.next().unwrap());

        Ok(Notes {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, notes_text: &str) -> Result<String, Error> {
        let notes: Notes = notes_text.parse().unwrap();
        Ok(notes.find_error_rate().to_string())
    }

    fn second_part(&self, notes_text: &str) -> Result<String, Error> {
        let notes: Notes = notes_text.parse().unwrap();
        Ok(notes.find_product_of_departure_fields().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_error_rate() {
        let notes_text = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let notes: Notes = notes_text.parse().unwrap();

        assert_eq!(71, notes.find_error_rate());
    }

    #[test]
    fn test_example_row_order() {
        let notes_text = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let notes: Notes = notes_text.parse().unwrap();

        assert_eq!(vec!["row", "class", "seat"], notes.find_field_order());
    }
}
