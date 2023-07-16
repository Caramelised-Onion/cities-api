1. ```docker-compose up```
2. cargo install sqlx-cli
3. sqlx migrate run

## Adding in country data
1. Download dataset [here](https://thematicmapping.org/downloads/world_borders.php)
2. Install shp2pgsql
3. Run ```shp2pgsql -s 4326 TM_WORLD_BORDERS_SIMPL-0.3.shp | psql -h localhost -d postgres -U postgres -p 5431```
4. Connect to db and run ```ALTER TABLE "tm_world_borders_simpl-0.3" RENAME TO countries;```