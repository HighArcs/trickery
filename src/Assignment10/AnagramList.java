import java.util.ArrayList;

public class AnagramList {
    private final ArrayList<String> anagrams = new ArrayList<>();

    public AnagramList(String word) {
        completeAnagrams(word, 0, word.length() - 1);
        sortAnagrams();
    }

    public void completeAnagrams(String s, int l, int r) {
        if (l == r) {
            if (!this.anagrams.contains(s)) {
                this.anagrams.add(s);
            }
        } else {

            for (int i = l; i <= r; i++) {
                s = swap(s, l, i);
                completeAnagrams(s, l + 1, r);
                s = swap(s, l, i);
            }
        }
    }

    public String swap(String a, int i, int j) {
        char temp;
        char[] charArray = a.toCharArray();
        temp = charArray[i];
        charArray[i] = charArray[j];
        charArray[j] = temp;
        return String.valueOf(charArray);
    }

    private void sortAnagrams() {
        for (int i = 0; i < this.anagrams.size() - 1; i++) {
            int min_idx = i;
            for (int j = i + 1; j < this.anagrams.size(); j++) {
                if (this.anagrams.get(j).compareTo(this.anagrams.get(min_idx)) < 0) {
                    min_idx = j;
                }
            }

            final String temp = this.anagrams.get(min_idx);
            this.anagrams.set(min_idx, this.anagrams.get(i));
            this.anagrams.set(i, temp);
        }
    }

    public int searchAnagrams(String target) {
        for (int index = 0; index < this.anagrams.size(); index++) {
            if (this.anagrams.get(index).equals(target)) {
                return index;
            }
        }

        return -1;
    }

    // Used to get list of anagrams externally, do not remove
    public ArrayList<String> getAnagrams() {
        return anagrams;
    }
}
