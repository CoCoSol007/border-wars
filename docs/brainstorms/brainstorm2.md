# Brainstrom

## Contents 

1) [Soldier system](#system-de-soldats)
2) [Fighting](#fighting)
3) [Building](#batiment)
4) [Villager management](#villager-management)
5) [Conquest](#conquest)
6) [Main menu system](#system-du-menu-principale)


## Soldier system

- move management
    - case selection
        - check membership.
    - display of free cases for moving.
    - Target square selection.
        - membership check + no enemies
    troop movement


## Fighting

- Attack side
    - selection of an enemy case
        - membership check
- Defense and attack sides
    - menu banner
        - Soldier selection + validation button
            - membership check
        - Start animation 
        - wait 1 second
        - Return animation without dead soldiers
            - previously calculated on server side
        - report + resource gain
        

## Building

- Upgradeable 
    - If you click on a building and then on the upgrade button
        - Check price
        - Verification of current bat level
        - Check if a worker is available
            - Casern 
                - Unlock a new troop
            - Outpost
                - Can tp more troops
            - Mine
                - \+ productive
            - Sawmill 
                - \+ productive
            - Breadings
                - \+ productive
            - Tower
                - \+ of reach or + of degat
            - Castle
                - Takes up one more square
                - Adds workers
                - Enhances everything with a small %.
            
- Destroy 
    - if on our territory
        - destroy building
        - add 1/2 of resources
     
- Build 
    - if at least 1 free worker 
    - if the price is not too high
    - if it's in our territory
        - building construction
        - distribute resources 

- Usage 
    - Check if the square belongs to us
        - Castle 
            - Villager management
            - *// TODO \\\\*
        - Outpost
            - Check which troops want to be sent
                - Errors
                - Teleport
        - Casern
            - Menu banner
                - All soldiers available for purchase
                    - Click
                        - If case alentoure free belonging to us
                        - Price check
                            - Error
                            - Creation

## Villagers management
- Castle clikced
    - Menu banner
    - List of professions and villagers
        - Change professions
            - Check current job
            - Check job to give
                - Mustn't be neutral
- List of jobs: Explorers, Worker or Neutral                    
## Conquest

- Choosing a case
    - Belonging

- Explorers
    - Stats: Speed (case/tours)
    - Explorer selection menu
        - Menu banner
        - Explorer list with
- Change membership
    - Previous membership
    - Membership to give
    

## Main menu system 
- Settings
- game mode
    - 1v1
    - 2v2
    - 1vIA
    - History
