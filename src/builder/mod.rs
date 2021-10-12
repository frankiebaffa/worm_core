use {
    crate::traits::{
        primarykey::PrimaryKeyModel,
        foreignkey::ForeignKey,
        foreignkey::ForeignKeyModel,
    },
    std::collections::HashMap,
};
trait WormErrorMatch<T, U: std::error::Error>: Sized {
    fn quick_match(self) -> Result<T, WormError>;
}
#[derive(Debug)]
pub enum WormError {
    NoRowsError,
    SQLError(rusqlite::Error),
}
impl std::fmt::Display for WormError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WormError::NoRowsError => {
                write!(f, "No rows found!")
            },
            WormError::SQLError(e) => {
                let msg = &format!("{}", e);
                f.write_str(msg)
            },
        }
    }
}
impl std::error::Error for WormError {}
impl<T> WormErrorMatch<T, rusqlite::Error> for Result<T, rusqlite::Error> {
    fn quick_match(self) -> Result<T, WormError> {
        return match self {
            Ok(s) => Ok(s),
            Err(e) => Err(WormError::SQLError(e)),
        };
    }
}
pub struct Query<'query, T> {
    select: String,
    from: String,
    join: Option<String>,
    clause: Option<String>,
    _value: Option<T>,
    params: HashMap<String, Box<&'query dyn rusqlite::ToSql>>,
}
impl<'query, T: PrimaryKeyModel> Query<'query, T> {
    pub fn select() -> Self {
        return Query {
            select: format!("select {}.*", T::ALIAS),
            from: format!("from {}.{} as {}", T::DB, T::TABLE, T::ALIAS),
            join: None,
            clause: None,
            _value: None,
            params: HashMap::new(),
        }
    }
    pub fn join_pk<U: ForeignKey<T>>(mut self) -> Self {
        let join_str;
        let dlim;
        if self.join.is_none() {
            join_str = String::new();
            dlim = String::new();
        } else {
            join_str = self.join.unwrap();
            dlim = String::from(" ");
        }
        self.join = Some(
            format!(
                "{}{}join {}.{} as {} on {}.{} = {}.{}",
                join_str, dlim,
                U::DB, U::TABLE, U::ALIAS,
                T::ALIAS, T::PRIMARY_KEY, U::ALIAS, U::FOREIGN_KEY
            )
        );
        return self;
    }
    pub fn join_and(mut self) -> Self {
        let join_str;
        let dlim;
        if self.join.is_none() {
            panic!("Cannot concatenate a join when no join exists");
        } else {
            join_str = self.join.unwrap();
            dlim = String::from(" ");
        }
        self.join = Some(
            format!(
                "{}{}and",
                join_str, dlim
            )
        );
        return self;
    }
    fn filter_join<'a, U: ForeignKey<T>>(mut self, op: &'a str, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        let join_str;
        let dlim;
        if self.join.is_none() {
            panic!("Cannot add another join constraint when there is no join");
        } else {
            join_str = self.join.unwrap();
            dlim = String::from(" ");
        }
        let param_name = format!(":param{}", self.params.len());
        self.params.insert(param_name.clone(), Box::new(value));
        self.join = Some(
            format!(
                "{}{}{}.{} {} {}",
                join_str, dlim,
                U::ALIAS, column,
                op, param_name,
            )
        );
        return self;
    }
    pub fn join_eq<'a, U: ForeignKey<T>>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter_join::<U>("=", column, value);
    }
    pub fn join_ne<'a, U: ForeignKey<T>>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter_join::<U>("!=", column, value);
    }
    pub fn join_gt<'a, U: ForeignKey<T>>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter_join::<U>(">", column, value);
    }
    pub fn join_lt<'a, U: ForeignKey<T>>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter_join::<U>("<", column, value);
    }
    pub fn join_ge<'a, U: ForeignKey<T>>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter_join::<U>(">=", column, value);
    }
    pub fn join_le<'a, U: ForeignKey<T>>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter_join::<U>("<=", column, value);
    }
    fn filter<'a>(mut self, op: &'a str, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        let clause_str;
        let dlim;
        if self.clause.is_none() {
            clause_str = String::new();
            dlim = String::from("where ");
        } else {
            clause_str = self.clause.unwrap();
            dlim = String::from(" ");
        }
        let param_name = format!(":param{}", self.params.len());
        self.params.insert(param_name.clone(), Box::new(value));
        self.clause = Some(
            format!(
                "{}{}{}.{} {} {}",
                clause_str, dlim,
                T::ALIAS, column,
                op, param_name,
            )
        );
        return self;
    }
    pub fn where_eq<'a>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter("=", column, value);
    }
    pub fn where_ne<'a>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter("!=", column, value);
    }
    pub fn where_gt<'a>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter(">", column, value);
    }
    pub fn where_lt<'a>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter("<", column, value);
    }
    pub fn where_ge<'a>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter(">=", column, value);
    }
    pub fn where_le<'a>(self, column: &'a str, value: &'query dyn rusqlite::ToSql) -> Self {
        return self.filter("<=", column, value);
    }
    fn concat<'a>(mut self, word: &'a str) -> Self {
        let clause_str;
        let dlim;
        if self.clause.is_none() {
            panic!("Cannot concatenate a clause when no clause exists");
        } else {
            clause_str = self.clause.unwrap();
            dlim = String::from(" ");
        }
        self.clause = Some(
            format!(
                "{}{}{}",
                clause_str, dlim, word
            )
        );
        return self;
    }
    pub fn and(self) -> Self {
        return self.concat("and");
    }
    pub fn or(self) -> Self {
        return self.concat("or");
    }
    pub fn execute(self) -> Result<Vec<T>, WormError>{
        let mut sql = format!("{} {}", self.select, self.from);
        if self.join.is_some() {
            let join = self.join.unwrap();
            sql.push_str(&format!(" {}", join));
        }
        if self.clause.is_some() {
            let clause = self.clause.unwrap();
            sql.push_str(&format!(" {}", clause));
        }
        // get query order of parameters
        let keys = self.params.keys();
        let mut key_indices: Vec<(usize, String)> = Vec::new();
        for key in keys {
            let index = sql.find(key).unwrap();
            sql = sql.replace(key, "?");
            key_indices.push((index, key.clone()));
        }
        key_indices.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut value_order = Vec::new();
        for key_index in key_indices {
            let key = key_index.1;
            let value = self.params.get(&key).unwrap();
            value_order.push(value);
        }
        let param = rusqlite::params_from_iter(value_order);
        let c = rusqlite::Connection::open("").quick_match()?;
        let mut stmt = c.prepare(&sql).quick_match()?;
        let mut rows = stmt.query(param).quick_match()?;
        let mut objs = Vec::new();
        while let Some(row) = rows.next().quick_match()? {
            objs.push(T::from_row(row)?);
        }
        return Ok(objs);
    }
    pub fn execute_row(self) -> Result<T, WormError> {
        let res = self.execute()?;
        if res.len() == 0 {
            return Err(WormError::NoRowsError);
        } else {
            let val = res.into_iter().nth(0).unwrap();
            return Ok(val);
        }
    }
}
pub trait JoinFK<T: PrimaryKeyModel>: ForeignKey<T> {
    fn join_fk(self) -> Self;
}
impl<'joinfk, T, U> JoinFK<U> for Query<'joinfk, U>
where
    T: PrimaryKeyModel,
    U: ForeignKey<T>,
{
    fn join_fk<T>(mut self) -> Self {
        let join_str;
        let dlim;
        if self.join.is_none() {
            join_str = String::new();
            dlim = String::new();
        } else {
            join_str = self.join.unwrap();
            dlim = String::from(" ");
        }
        self.join = Some(
            format!(
                "{}{}join {}.{} as {} on {}.{} = {}.{}",
                join_str, dlim,
                T::DB, T::TABLE, T::ALIAS,
                U::ALIAS, U::FOREIGN_KEY, T::ALIAS, T::PRIMARY_KEY
            )
        );
        return self;
    }
}
