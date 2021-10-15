use {
    crate::traits::{
        dbctx::DbCtx,
        primarykey::PrimaryKeyModel,
        foreignkey::ForeignKey,
    },
    std::{
        collections::HashMap,
        error::Error as StdError,
        fmt::Display as StdDisplay,
        fmt::Formatter as StdFormatter,
        fmt::Result as FmtResult,
    },
    rusqlite::{
        Error as SQLError,
        ToSql,
    }
};
trait WormErrorMatch<T, U>: Sized where U: StdError {
    fn quick_match(self) -> Result<T, WormError>;
}
#[derive(Debug)]
pub enum WormError {
    NoRowsError,
    SQLError(SQLError),
}
impl StdDisplay for WormError {
    fn fmt(&self, f: &mut StdFormatter) -> FmtResult {
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
impl StdError for WormError {}
impl<T> WormErrorMatch<T, SQLError> for Result<T, SQLError> {
    fn quick_match(self) -> Result<T, WormError> {
        return match self {
            Ok(s) => Ok(s),
            Err(e) => Err(WormError::SQLError(e)),
        };
    }
}
enum QueryType {
    Select,
    Update,
}
pub struct Query<'query, T> {
    query_type: QueryType,
    select: String,
    update: String,
    set: Option<String>,
    from: String,
    join: Option<String>,
    clause: Option<String>,
    _value: Option<T>,
    params: HashMap<String, Box<&'query dyn ToSql>>,
}
impl<'query, T> Query<'query, T> where T: PrimaryKeyModel {
    pub fn select() -> Self {
        return Query {
            query_type: QueryType::Select,
            select: format!("select {}.*", T::ALIAS),
            update: String::new(),
            set: None,
            from: format!("from {}.{} as {}", T::DB, T::TABLE, T::ALIAS),
            join: None,
            clause: None,
            _value: None,
            params: HashMap::new(),
        };
    }
    pub fn update() -> Self {
        return Query {
            query_type: QueryType::Update,
            select: String::new(),
            update: format!("update {}.{}", T::DB, T::TABLE),
            set: None,
            from: format!("from {}.{} as {}", T::DB, T::TABLE, T::ALIAS),
            join: None,
            clause: None,
            _value: None,
            params: HashMap::new(),
        };
    }
    pub fn set<'a>(mut self, column: &'a str, value: &'query dyn ToSql) -> Self {
        let dlim;
        let set;
        if self.set.is_none() {
            set = String::new();
            dlim = "where ";
        } else {
            set = self.set.unwrap();
            dlim = ", ";
        }
        let param_name = format!(":param{}", self.params.len());
        self.params.insert(param_name.clone(), Box::new(value));
        self.set = Some(format!(
            "{}{}{}.{} = {}",
            set, dlim, T::ALIAS, column, param_name
        ));
        return self;
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
    fn filter_join<'a, U>(
        mut self,
        op: &'a str,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
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
    pub fn join_eq<'a, U>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
        return self.filter_join::<U>("=", column, value);
    }
    pub fn join_ne<'a, U>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
        return self.filter_join::<U>("!=", column, value);
    }
    pub fn join_gt<'a, U>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
        return self.filter_join::<U>(">", column, value);
    }
    pub fn join_lt<'a, U>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
        return self.filter_join::<U>("<", column, value);
    }
    pub fn join_ge<'a, U>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
        return self.filter_join::<U>(">=", column, value);
    }
    pub fn join_le<'a, U>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self where U: ForeignKey<T> {
        return self.filter_join::<U>("<=", column, value);
    }
    fn filter<'a>(
        mut self,
        op: &'a str,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
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
    pub fn where_eq<'a>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
        return self.filter("=", column, value);
    }
    pub fn where_ne<'a>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
        return self.filter("!=", column, value);
    }
    pub fn where_gt<'a>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
        return self.filter(">", column, value);
    }
    pub fn where_lt<'a>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
        return self.filter("<", column, value);
    }
    pub fn where_ge<'a>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
        return self.filter(">=", column, value);
    }
    pub fn where_le<'a>(
        self,
        column: &'a str,
        value: &'query dyn ToSql
    ) -> Self {
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
    pub fn query_to_string(&self) -> String {
        let mut sql;
        match self.query_type {
            QueryType::Select => sql = format!("{} {}", self.select, self.from),
            QueryType::Update => {
                if self.set.is_none() {
                    panic!("Cannot create an update statement without any set values");
                }
                sql = format!("{} {} {}", self.update, self.set.clone().unwrap(), self.from);
            },
        }
        if self.join.is_some() {
            let join = self.join.clone().unwrap();
            sql.push_str(&format!(" {}", join));
        }
        if self.clause.is_some() {
            let clause = self.clause.clone().unwrap();
            sql.push_str(&format!(" {}", clause));
        }
        return sql;
    }
    pub fn execute(self, db: &mut impl DbCtx) -> Result<Vec<T>, WormError>{
        let mut sql = self.query_to_string();
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
        let c = db.use_connection();
        let mut objs = Vec::new();
        match self.query_type {
            QueryType::Select => {
                let mut stmt = c.prepare(&sql).quick_match()?;
                let mut rows = stmt.query(param).quick_match()?;
                while let Some(row) = rows.next().quick_match()? {
                    objs.push(T::from_row(row).quick_match()?);
                }
            },
            QueryType::Update => {
                let id;
                {
                    let mut tx = c.transaction().quick_match()?;
                    {
                        let sp = tx.savepoint().quick_match()?;
                        sp.execute(&sql, param).quick_match()?;
                        id = sp.last_insert_rowid();
                    }
                }
                objs = Query::<T>::select()
                    .where_eq(T::PRIMARY_KEY, &id)
                    .execute(db)?;
            },
        }
        return Ok(objs);
    }
    pub fn execute_row(self, db: &mut impl DbCtx) -> Result<T, WormError> {
        let res = self.execute(db)?;
        if res.len() == 0 {
            return Err(WormError::NoRowsError);
        } else {
            let val = res.into_iter().nth(0).unwrap();
            return Ok(val);
        }
    }
}
pub trait JoinFK<T, U> where T: PrimaryKeyModel, U: ForeignKey<T> {
    fn join_fk(self) -> Self;
}
impl<'joinfk, T, U> JoinFK<T, U> for Query<'joinfk, U>
where
    T: PrimaryKeyModel,
    U: ForeignKey<T>,
{
    fn join_fk(mut self) -> Self {
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
