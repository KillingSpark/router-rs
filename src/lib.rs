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
            path: vec!["ab", "cd"],
        };
        let route2 = router::Route {
            path: vec!["abc", "*", ":cde"],
        };

        let route2_request = router::Route {
            path: vec!["abc", "this_is_ingnored_by_wildcard", "this_is_a_param"],
        };

        println!("########################################");
        println!("########################################");
        println!("########################################");
        r.add_route(&route, 10);
        println!("########################################");
        r.add_route(&route2, 20);
        println!("########################################");
        println!("########################################");

        let (x, params) = r.route(&route2_request).unwrap();
        assert!(*x == 20);
        assert_eq!(
            params.get(&":cde".to_owned()),
            Some(&"this_is_a_param".to_owned())
        );

        struct Beep {
            a: u32,
        };

        let mut r: router::Router<&mut Beep> = router::Router {
            tree: router::Tree::Wildcard(Vec::new()),
        };
        let mut b = Beep { a: 10 };
        r.add_route(&route, &mut b);
        let (x, _) = r.route(&route).unwrap();

        assert!(x.a == 10);
        x.a = 100;
        assert!(b.a == 100);
    }
}
