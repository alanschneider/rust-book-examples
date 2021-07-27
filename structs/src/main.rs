//////////////////////////////////////////////////
// General Notes:
//
// - In rust, struct mutability is all or nothing.
//   You can't define mutability on certain fields.
//

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// Example of a unit-like struct
//

struct AUnitLikeStruct {
}

// Use field init shorthand for email and username
//
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let email = String::from("some@user.com");
    let username = String::from("someuser");

    let _user1 = build_user(email, username);

    // Create instance using struct update syntax
    //
    let _user2 = User {
        email: String::from("someother@user.com"),
        username: String::from("someotherguy"),
        .._user1
    };


    // Tuple structs
    //
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let _black = Color(0, 0, 0);
        let _origin = Point(0, 0, 0);
    }

    let _a_unit_like_struct = AUnitLikeStruct {};
}
