package PokemonBattle;

import java.util.Scanner;

public class Pokemon {
    private int health, strength, speed, direction, x, y, z;
    private String name;
    private static Scanner scan = new Scanner(System.in);

    public Pokemon(String name, int health, int strength, int speed) {
        this.name = name;
        this.health = health;
        this.strength = strength;
        this.speed = speed;
    }

    /** Gets x coordinate of current player */
    public int get_x() {
        return this.x;
    }

    /** Gets y coordinate of current player */
    public int get_y() {
        return this.y;
    }

    /** Gets z coordinate of current player */
    public int get_z() {
        return this.z;
    }

    /** Gets health points (hp) of current player */
    public int get_hp() {
        return this.health;
    }

    /** Gets direction of current player */
    public int get_direction() {
        return this.direction;
    }

    /** To set health */
    public void set_hp(final int hp) {
        if (hp < 0) {
            this.health = 0;
        } else {
            this.health = hp;
        }
    }

    public static final int dir_north = 1;
    public static final int dir_south = 2;
    public static final int dir_left = 3;
    public static final int dir_right = 4;

    public void move(final int direction, final int units) {
        if (direction == Pokemon.dir_north) {
            this.x += units;
        } else if (direction == Pokemon.dir_south) {
            this.x -= units;
        } else if (direction == Pokemon.dir_left) {
            this.y += units;
        } else if (direction == Pokemon.dir_right) {
            this.y -= units;
        }
    }

    public void set_direction(int direction) {
        if (direction >= 1 && direction <= 6) {
            this.direction = direction;
        }
    }

    private int chance(int min, int max) {
        return (int) (Math.random() * (max - min + 1) + min);
    }

    public static final String[] pokemon_names = { "Pikachu", "Charizard", "Bulbasaur", "Sylveon", "[enemy pokemon]" };

    /**
     * Creates an enemy Pokemon that encounters our own
     * -> The player is given the choice to battle or run away
     */
    private void calculate_battle_chance() {
        int chance = this.chance(50, 100);

        if (chance > 50) {
            Pokemon enemy = new Pokemon(Pokemon.pokemon_names[(int) (Math.random() * Pokemon.pokemon_names.length)],
                    this.chance(0, 100), this.chance(0, 100),
                    this.chance(0, 100));

            System.out.println("You have ran into a " + enemy.name + "!");

            System.out.println("Type 1 to fight or 2 to run away:");

            final int choice = Pokemon.scan.nextInt();

            if (choice == 2) {
                System.out.println("You decided to run away");
                this.calculate_move();
            } else if (choice == 1) {
                System.out.println("You decided to fight");
                Pokemon.battle(this, enemy);
                this.calculate_move();
            }
        } else {
            this.calculate_move();
        }
    }

    private void calculate_move() {
        System.out.println("\nWhich way would you like to move?");
        System.out.println("Forward: 1, Backward: 2, Left: 3, Right: 4, Quit: 5");
        final int new_dir = Pokemon.scan.nextInt();

        this.set_direction(new_dir);
        this.move(this.direction, 1);

        if (new_dir == 1) {
            System.out.println("\nYou have moved forward.");
        } else if (new_dir == 2) {
            System.out.println("\nYou have moved backward.");
        } else if (new_dir == 3) {
            System.out.println("\nYou have moved left.");
        } else if (new_dir == 4) {
            System.out.println("\nYou have moved right.");
        } else if (new_dir == 5) {
            System.exit(0);
        }

        this.calculate_battle_chance();
        System.out.println("\n");
    }

    public static void battle(Pokemon pokemon1, Pokemon pokemon2) {
        System.out.println(pokemon1.name + " begins the fight against "
                + pokemon2.name);

        while (pokemon1.health > 0 || pokemon2.health > 0) {
            if (pokemon1.health <= 0 || pokemon2.health <= 0) {
                break;
            }

            pokemon2.health -= pokemon1.strength;
            System.out.println(pokemon1.name + " does " + pokemon1.strength + " damage to " + pokemon2.name + " and "
                    + pokemon2.name + " has " + pokemon2.health + " health left.");
            pokemon1.health -= pokemon2.strength;

            System.out.println(pokemon2.name + " does " + pokemon2.strength + " damage to " + pokemon1.name + " and "
                    + pokemon1.name + " has " + pokemon1.health + " health left.");
        }

        if (pokemon1.health <= 0) {
            System.out.println(pokemon1.name + " has lost the fight");
        } else if (pokemon2.health <= 0) {
            System.out.println(pokemon2.name + " has lost the fight");
        }
    }

    public void start() {
        this.calculate_move();
    }
}
