use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};
use sqlite::Error as SqErr;


pub struct UserBase{
    fname:String,
}

impl UserBase{
    pub fn add_user(&self, u_name:&str, p_word:&str)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;
        let hpass=bcrypt::hash(p_word,DEFAULT_COST)?;
        let mut st= conn.prepare("insert into users(u_name, p_word) values (?,?);")?;
        st.bind(1,u_name)?;
        st.bind(2,&hpass as &str)?;
        st.next()?;
        Ok(())
    }

    pub fn pay(&self, u_from:&str, u_to:&str, amount:i64)->Result<(),UBaseErr>{
        let conn=sqlite::open(&self.fname)?;
        let mut st= conn.prepare("insert into transactions (u_from, u_to, t_date, t_amount) values(?,?,datetime(\"now\"),?);")?;
        st.bind(1,u_from)?;
        st.bind(2,u_to)?;
        st.bind(3,amount)?;
        st.next()?;
        Ok(())
    }
}
    

#[derive(Debug )]
pub enum UBaseErr{
    DbErr(SqErr),
    HashError(BcryptError)
}

impl From<SqErr> for UBaseErr{
    fn from(s:SqErr)->Self{
        UBaseErr::DbErr(s)
    }
}

impl From<BcryptError> for UBaseErr{
    fn from(b:BcryptError)->Self{
        UBaseErr::HashError(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlite::State;

    #[test]
    fn test_add_user() {
        let user_base = UserBase { fname: String::from("data/users.db") };
        let username = "testuser";
        let password = "password123";

        assert!(user_base.add_user(username, password).is_ok());

        let conn = sqlite::open(&user_base.fname).unwrap();
        let mut stmt = conn.prepare("SELECT u_name FROM users WHERE u_name = ?;").unwrap();
        stmt.bind(1, username).unwrap();
        assert_eq!(stmt.next().unwrap(), State::Row);
    }

    #[test]
    fn test_pay() {
        let user_base = UserBase { fname: String::from("data/users.db") };
        user_base.add_user("user1", "pass1").unwrap();
        user_base.add_user("user2", "pass2").unwrap();

        assert!(user_base.pay("user1", "user2", 100).is_ok());

        let conn = sqlite::open(&user_base.fname).unwrap();
        let mut stmt = conn.prepare("SELECT * FROM transactions WHERE u_from = ? AND u_to = ?;").unwrap();
        stmt.bind(1, "user1").unwrap();
        stmt.bind(2, "user2").unwrap();
        assert_eq!(stmt.next().unwrap(), State::Row);
    }
}

fn main() {
    let user_base = UserBase { fname: String::from("data/users.db") };
    let username = "testuser1";
    let password = "password123";

    match user_base.add_user(username, password) {
        Ok(_) => println!("User added successfully"),
        Err(e) => println!("Error adding user: {:?}", e),
    }
}