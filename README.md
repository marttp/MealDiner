# MealDiner

The solid system for your restaurant. Implemented by Rust programming languages.

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

TBC