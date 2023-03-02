package Assignment10;

import java.util.ArrayList;

public class AnagramList {
    private final ArrayList<String> anagrams;

    public AnagramList(String word) {
        anagrams = new ArrayList<String>();
        completeAnagrams(word);
        sortAnagrams();
    }

    public void completeAnagrams(String word) {
        if (word.length() <= 2) {
            if (word.length() == 2) {
                this.anagrams.add(word);
                this.anagrams.add(word.substring(1) + word.substring(0, 1));
            } else {
                this.anagrams.add(word);
            }
        }

        
    }

    private void sortAnagrams() {
        for (int i = 0; i < this.anagrams.size(); i++) {
            int min = i;
            for (int j = i + 1; j < this.anagrams.size(); i++) {
                if (this.anagrams.get(j).compareTo(this.anagrams.get(min)) < 0) {
                    min = j;
                }
            }

            String temp = this.anagrams.get(min);
            this.anagrams.set(min, this.anagrams.get(i));
            this.anagrams.set(i, temp);
        }
    }

    public int searchAnagrams(String target) {
        return -1;
    }

    // Used to get list of anagrams externally, do not remove
    public ArrayList<String> getAnagrams() {
        return anagrams;
    }
}
