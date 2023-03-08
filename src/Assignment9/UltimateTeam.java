// package Assignment9;

import java.util.ArrayList;

public class UltimateTeam {
    public final ArrayList<UltimatePlayer> players;
    public final ArrayList<Coach> coaches;

    public UltimateTeam(final ArrayList<UltimatePlayer> players, final ArrayList<Coach> coaches) {
        this.players = players;
        this.coaches = coaches;
    }

    public final String getCutters() {
        String out = "";

        for (int index = 0; index < this.players.size(); index++) {
            UltimatePlayer player = this.players.get(index);
            if (player.position.equals("cutter")) {
                out += player.toString() + "\n";
            }
        }

        return out;
    }

    public final String getHandlers() {
        String out = "";

        for (int index = 0; index < this.players.size(); index++) {
            UltimatePlayer player = this.players.get(index);
            if (player.position.equals("handler")) {
                out += player.toString() + "\n";
            }
        }

        return out;
    }

    public final String toString() {
        String out = "COACHES\n";

        for (int i = 0; i < this.coaches.size(); i++) {
            Coach coach = this.coaches.get(i);
            out += coach.toString() + "\n";
        }

        out += "\nPLAYERS\n";

        for (int i = 0; i < this.players.size(); i++) {
            UltimatePlayer player = this.players.get(i);
            out += player.toString() + "\n";
        }

        return out;
    }
}