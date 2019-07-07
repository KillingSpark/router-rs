mod router;

#[cfg(test)]
mod tests {
    use crate::router;
    #[test]
    fn it_works() {
        let mut r: router::Router<u32> = router::Router {
            tree: router::Tree::Wildcard(Vec::new()),
        };
        let route = router::Route {
            Path: vec!["ab", "cd"],
        };
        let route2 = router::Route {
            Path: vec!["abc", "*", ":cde"],
        };

        let route2_request = router::Route {
            Path: vec!["abc", "this_is_ingnored_by_wildcard", "this_is_a_param"],
        };

        println!("########################################");
        println!("########################################");
        println!("########################################");
        r.add_route(&route, 10);
        println!("########################################");
        r.add_route(&route2, 20);
        println!("########################################");
        let (x, params) = r.route(&route2_request).unwrap();
        println!("Routed to: {}", x);
        println!(
            "Parameter found : {}",
            params.get(&":cde".to_owned()).unwrap()
        );
        println!("########################################");
        println!("########################################");

        struct beep {
            A: u32,
        };

        let mut r: router::Router<&mut beep> = router::Router {
            tree: router::Tree::Wildcard(Vec::new()),
        };
        let mut b = beep { A: 10 };
        r.add_route(&route, &mut b);
        let (x, _) = r.route(&route).unwrap();
        x.A = 100;
        println!("{}, should be 100", b.A);
    }
}
