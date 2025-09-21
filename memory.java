import java.util.*;

public class test {

    public static void main(String[] args) {
        int[] arr = new int[3]; // allocate memory dynamically
        arr[0] = 10;
        arr[1] = 20;
        arr[2] = 30;

        System.out.println("Java array allocated");

        arr = null; // remove reference
        System.gc(); // suggest garbage collection
        System.out.println("Java: garbage collector reclaims memory automatically");
    }

}
