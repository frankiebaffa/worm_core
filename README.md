# wORM

A light(w)eight (O)bject (R)elational (M)apper.

_ _ _

## Procedural Macros

Procedural macro support can be added with [wORM Derive](https://github.com/frankiebaffa/worm_derive).

## Implementation

The following is how to implement **wORM** without using [wORM Derive](https://github.com/frankiebaffa/worm_derive). Keep in mind, I **do not** recommend using this method of implementing **wORM**. I recommend also using **wORM Derive** as it leads to much less boilerplate code per table. If you wish to compare the difference, please check out the [implementation](https://github.com/frankiebaffa/worm_derive#user-content-implementation) using **wORM Derive**.

Assume we have a database table named `AlbumTrackArtists` in a database file named `DecibelDb.db` whose creation script look something like this:

```sql
create table DecibelDb.AlbumTrackArtists
	(
		Id integer not null primary key autoincrement
	,	Artist_Id integer not null
	,	AlbumTrack_Id integer not null
	,	ArtistType_Id integer not null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	,	foreign key (Artist_Id) references Artists (Id)
	,	foreign key (AlbumTrack_Id) references AlbumTracks (Id)
	,	foreign key (ArtistType_Id) references ArtistTypes (Id)
	);
create unique index DecibelDb.AlbumTrackArtistsUnique on AlbumTrackArtists
	(
		Artist_Id
	,	AlbumTrack_Id
	,	ArtistType_Id
	)
where Active = 1;
```

Rust's *struct* equivalent would look like this:

```rust
use chrono::{DateTime, Local};
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

Notice that the **AlbumTrackArtists** table contains a primary key? We can now implement the trait **PrimaryKey** from worm to auto-generate sql functions based on this. Everything that implements the **PrimaryKey** trait receives an implementation for **PrimaryKeyModel** which contains the useful boilerplate function `get_by_id()`.

```rust
use worm::traits::primarykey::PrimaryKey;
impl PrimaryKey for AlbumTrackArtist {
	const PRIMARY_KEY: &'static str;
	fn get_id(&self) -> i64 {
		return self.id.clone();
	}
}
```

The database table also contains an `Active` flag. This is something that I commonly use in my SQL tables to allow deletion of entries without actually removing the record from the database. With this in mind, we can then implement the useful trait: **ActiveFlag**. Anything which implements this trait also gets an implementation of **ActiveFlagModel** which yields the function `get_all_active()`. This queries the table for all objects which have an active value of 1.

```rust
use worm::traits::activeflag::ActiveFlag;
impl ActiveFlag for AlbumTrackArtist {
	const ACTIVE: &'static str = "Active";
	fn get_active(&self) -> bool {
		return self.active.clone();
	}
}
```

The table **AlbumTrackArtists** also contains several foreign IDs, `Artist_Id`, `AlbumTrack_Id`, and `ArtistType_Id`. Assuming that these are already existing models in our rust code (they must at least implement **PrimaryKey**), we can then implement the trait **ForeignKey**. This will automatically implement **ForeignKeyModel** for our type which will give us access to `get_all_by_fk()` and `get_fk(&self)`. The former will get every object of the implementing type which has the given foreign key value for given type parameter. The latter returns the object of the type parameter whose primary key is the value of the associated foreign key field in the implemented object.

```rust
use worm::traits::foreignkey::ForeignKey;
impl ForeignKey<Artist> for AlbumTrackArtists {
	const FOREIGN_KEY: &'static str = "Album_Id";
	const FOREIGN_KEY_PARAM: &'static str = ":album_id";
	fn get_fk_value(&self) -> i64 {
		return self.album_id.clone();
	}
}
```

We can then repeat this last step for the remaining foreign key types.

Once again, I urge you to check out the [implementation section](https://github.com/frankiebaffa/worm_derive#user-content-implementation) for **wORM Derive**. It implements the exact same traits as this example and saves a ton of time typeing boilerplate code.
