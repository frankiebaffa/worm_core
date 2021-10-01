# wORM

A light(w)eight (O)bject (R)elational (M)apper.

_ _ _

## Procedural Macros

Procedural macro support can be added with [wORM Derive](https://github.com/frankiebaffa/worm_derive).

## Implementation

The following is how to implement **wORM** without using [wORM Derive](https://github.com/frankiebaffa/worm_derive). Keep in mind, I **do not** recommend using this method of implementing **wORM**. I recommend also using **wORM Derive** as it leads to much less boilerplate code per table. If you wish to compare the difference, please checkout the readme inlcuded in **wORM Derive**.

Assume we have a database table named `AlbumTrackArtists` in a database file named `DecibelDb.db` whose creation script look something like this:

```sql
create table DecibelDb.AlbumTrackArtists
	(
		Id integer not null primary key autoincrement
	,	Name text not null
	,	Bio text null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	);
```

Rust's *struct* equivalent would look like this:

```rust
pub struct AlbumTrackArtist {
    id: i64,
    artist_id: i64,
    albumtrack_id: i64,
    artisttype_id: i64,
    active: bool,
    createddate: DateTime<Local>,
    lasteditdate: DateTime<Local>,
}
```

Now implement the trait **DbModel** for the struct **AlbumTrackArtist**:

```rust
use rusqlite::{Error, Row};
use worm::traits::dbmodel::DbModel;
use worm::traits::helpers::ColumnValue;
impl DbModel for AlbumTrackArtist {
    const DB: &'static str = "DecibelDb";
    const TABLE: &'static str = "AlbumTrackArtists";
    const ALIAS: &'static str = "albumtrackartists";
    fn from_row(row: &Row) -> Result<Self, Error> {
		let id = row.value("Id")?;
		let artist_id = row.value("Artist_Id")?;
		let albumtrack_id = row.value("Albumtrack_Id")?;
		let artisttype_id = row.value("Artisttype_Id")?;
		let active = row.value("Active")?;
		let createddate = row.value("CreatedDate")?;
		let lasteditdate = row.value("LastEditDate")?;
		return Ok(AlbumTrackArtist { id, artist_id, albumtrack_id, artisttype_id, active, createddate, lasteditdate, });
	}
}
```
