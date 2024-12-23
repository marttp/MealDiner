# MealDiner

The solid system for your restaurant. Implemented by Rust programming languages.

## Developer

Thanaphoom Babparn (Mart) - [LinkedIn](https://www.linkedin.com/in/thanaphoom-babparn/?locale=en_US)

## How to run simulation

1. Open terminal
2. Start backend
    ````shell
    cd server
    cargo run
    ````
3. Open another terminal
4. Start client simulation
    ````shell
    cd client
    # 200 RPS
    cargo run -- --interval-secs 1 --max-rps 200
    ````
   You can control the amount of request.
   * **interval-secs**: control sleep period for spawning thread => Mostly use 1 for imitate per second
   * **max-rps**: how many tokio you want to spawn? 1000 means 1000 processes

## How to run tests & test coverage

### Run test only

1. Move to backend application
    ```shell
    cd server
    ```
2. Run all test
    ```shell
    cargo test
    ```

### Test Coverage

1. Install required tools first
    ```shell
    rustup component add llvm-tools-preview
    cargo install cargo-llvm-cov
    ```
2. Move to backend application
    ```shell
    cd server
    ```
   
3. Run test coverage

    ```shell
    cargo llvm-cov
    ```

## Business purpose

* A restaurant application which accepts menu items from various serving staff in the restaurant. 
* Must then store the item along with a cooking time for the item to be completed.
* Must be able to give a quick snapshot of any or all items on its list at any time. 
* Able to remove specific orders from the list of orders on demand.

## Main design

* REST API application design

## Grouping requirements

### Server

#### Client interaction
* Query request - show all items for a specified table number.
* Query request - show a specified item for a specified table number.
* Creation request - store the item, the table number, and how long the item will take to cook.
* Deletion request - remove a specified item for a specified table number.

#### Traffic pattern
* Accept at least 10 simultaneous incoming add/remove/query requests.
* The client MAY limit the number of specific tables in its requests to a finite set (at least 100).

#### Time criteria
* Assign a length of time for the item to prepare as a random time between 5-15 minutes. 
  * Note - the time does not have to be counted down in real time, only upon item creation and then removed with the item upon item deletion).

### Client

* The client (tablets devices) able to
  * add one or more items with a table number.
  * remove an item for a table.
  * query the items still remaining for a table.

## Design Ideas

### Business Example

I am going to use popular food chain restaurant in Japan which has 2,000 branches covered in Japan as example.

For our use-case, I want to assume we have 1,000 branches and with 10 tables on each. However, Assume RPS/TPS is at least 100 RPS.
* 1,000 branches x 10 tables = 10,000 tables
* 100 simultaneous requests per second

![overview scenario.jpg](files/overview%20scenario.jpg)

### API Design

```text
* Query request - show all items for a specified table number.
* Query request - show a specified item for a specified table number.
* Creation request - store the item, the table number, and how long the item will take to cook.
* Deletion request - remove a specified item for a specified table number.
```

This can refer to 4 API without update capability

- `GET /tables/:id/orders` - Get list of menu on specify table
- `GET /tables/:id/orders/:order_id` - Get specify item for order id in that table
- `POST /orders` - Order new food. Accept multiple menus. Table specify in payload. Also, random cooking time internally on backend
- `DELETE /tables/:id/orders/:order_id` - Delete specify item for order id in that table

For smoother in simulation, additional endpoint

- `GET /health` - Status of backend
- `GET /menus` - Get list of menu. For this simulation, only 3 menus are allowed.
  - Ramen
  - Beef rice
  - Beer
- `GET /configs` - Just for initialization on client
  - How many tables? start - end

### Client simulation logic

1. Get configs & menus
2. Get orders of the table
   * If table empty -> Order
   * If table contains at least 1 menu, Random below options
     * Order more
     * Random delete
     * Random get specify order id

![client-simulation.jpg](files/client-simulation.jpg)

## Example Result

### API (Hoppscotch)

[MealDiner.json](files/MealDiner.json)

### Server

![server-result.jpg](files/server-result.jpg)

### Client

![client-result.jpg](files/client-result.jpg)


## Build & Run container

### Old `docker build`

``shell
docker build -t mealdiner:0.0.1 .
``

### With `buildx`

Install buildx first

```shell
brew install docker-buildx

mkdir ~/.docker/cli-plugins

ln -sfn $(which docker-buildx) ~/.docker/cli-plugins/docker-buildx

docker buildx install
```

Then use this command

```shell
docker buildx build -t mealdiner:0.0.1 .
```

Then run

```shell
docker run -d -p 8080:8080 -e SERVER_PORT=8080 -e AVAILABLE_TABLES=10000 mealdiner:0.0.1
```

## Contact
If you have any questions, feel free to reach out to me at below contact
- Facebook: [Thanaphoom Babparn](https://www.facebook.com/thanaphoom.mart/)
- FB Page: [TP Coder](https://www.facebook.com/tpcoder)
- LinkedIn: [Thanaphoom Babparn](https://www.linkedin.com/in/thanaphoom-babparn/)
- Website: [TP Coder — Portfolio](https://portfolio.tpcoder.dev/)
