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
    public static final int dir_up = 3;
    public static final int dir_down = 4;
    public static final int dir_east = 5;
    public static final int dir_west = 6;

    public void move(final int direction, final int units) {
        if (direction == Pokemon.dir_north) {
            this.x += units;
        } else if (direction == Pokemon.dir_south) {
            this.x -= units;
        } else if (direction == Pokemon.dir_up) {
            this.y += units;
        } else if (direction == Pokemon.dir_down) {
            this.y -= units;
        } else if (direction == Pokemon.dir_east) {
            this.z += units;
        } else if (direction == Pokemon.dir_west) {
            this.z -= units;
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

    private int calculate_battle_chance() {
        int chance = this.chance(50, 100);

        if (chance > 50) {
            Pokemon enemy = new Pokemon("every borrowck error ever", this.chance(0, 100), this.chance(0, 100),
                    this.chance(0, 100));

            System.out.println("\u001b[33mYou have ran into a \u001b[32m" + enemy.name + "\u001b[0m!");
        }

        return 0;
    }
}
