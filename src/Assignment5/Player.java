// @ignore Java(536871240)
package Assignment5;

public class Player {
    private static final int NORTH = 1;
    private static final int SOUTH = 2;
    private static final int UP = 3;
    private static final int DOWN = 4;
    private static final int EAST = 5;
    private static final int WEST = 6;

    private static int numPlayers = 0;
    private int x;
    private int y;
    private int z;
    private int direction;
    private int hp;
    private String name;

    public Player(String name, int x, int y, int z, int health, int direction) {
        System.out.println(Player.numPlayers);
        Player.numPlayers++;
        if (hp < 0) {
            hp = 0;
        }

        if (direction < 1 || direction > 6) {
            direction = 1;
        }

        this.name = name;
        this.x = x;
        this.y = y;
        this.z = z;
        this.hp = health;
        this.direction = direction;
    }

    public Player(String name, int x, int y, int z) {
        this(name, x, y, y, 20, Player.NORTH);
    }

    public Player() {
        this("P" + Player.numPlayers + 1, 0, 0, 0);

    }

    public static int getNumPlayers() {
        return Player.numPlayers;
    }

    public String getName() {
        return this.name;
    }

    public int getX() {
        return this.x;
    }

    public int getY() {
        return this.y;
    }

    public int getZ() {
        return this.z;
    }

    public int getHp() {
        return this.hp;
    }

    public int getDirection() {
        return this.direction;
    }

    public String toString() {
        return "Name: " + this.name + "\nHealth: " + this.hp + "\nCoordinates: X " + this.x + " Y " + this.y + " Z "
                + this.z + "\nDirection: " + this.direction;
    }

    public void setHp(int hp) {
        if (hp < 0) {
            hp = 0;
        }

        this.hp = hp;
    }

    public void setDirection(int direction) {
        if (direction > 1 && direction < 6) {
            this.direction = direction;
        }
    }

    public void move(int direction, int units) {
        if (direction == 1) {
            this.x += units;
        } else if (direction == 2) {
            this.x -= units;
        } else if (direction == 3) {
            this.y += units;
        } else if (direction == 4) {
            this.y -= units;
        } else if (direction == 5) {
            this.z += units;
        } else if (direction == 6) {
            this.z -= units;
        }
    }

    public void teleport(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public void teleport(Player player) {
        this.teleport(player.x, player.y, player.z);
    }

    public void attack(Player player, int damage) {
        player.hp -= damage;

        // saturating underflow
        if (player.hp < 0) {
            player.hp = 0;
        }

        this.hp += (damage / 2);
    }

    public double getDistance(int x, int y, int z) {
        return Math.sqrt(Math.pow(x - this.x, 2) + Math.pow(y - this.y, 2) + Math.pow(z - this.z, 2));
    }

    public double getDistance(Player player) {
        return this.getDistance(player.x, player.y, player.z);
    }

}
