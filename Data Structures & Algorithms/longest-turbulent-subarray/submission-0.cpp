enum Condition { 
    None,
    OddLesser, 
    OddGreater
};

class Solution {
public:
    int maxTurbulenceSize(vector<int>& arr) {
        int max_length = 1;
        int cur_length = 1;

        int n = arr.size();
        Condition cur_condition = None;

        for (int k = 0; k < n - 1; k++) {
            int cur_ele = arr[k];
            int next_ele = arr[k + 1];

            if (cur_condition == None) {
                // evaluating condition
                if (k % 2 == 0 && cur_ele > next_ele || k % 2 != 0 && cur_ele < next_ele) {
                    cur_condition = OddLesser;
                } else {
                    cur_condition = OddGreater;
                }
            }

            int odd_idx = (k % 2 != 0) ? k : k + 1;
            int even_idx = (k % 2 == 0) ? k : k + 1;

            if (cur_condition == OddLesser) {
                // Odd_Lesser
                if (arr[odd_idx] < arr[even_idx]) {
                    cur_length = cur_length + 1;
                } else {
                    if (arr[odd_idx] > arr[even_idx]) {
                        cur_condition = OddGreater;
                        cur_length = 2;
                    } else {
                        cur_condition = None;
                        cur_length = 1;
                    }
                };
            } else {
                // Odd_Greater
                if (arr[odd_idx] > arr[even_idx]) {
                    cur_length = cur_length + 1;
                } else {
                    if (arr[odd_idx] < arr[even_idx]) {
                        cur_condition = OddLesser;
                        cur_length = 2;
                    } else {
                        cur_condition = None;
                        cur_length = 1;
                    }
                };
            }

            if (cur_length > max_length) {
                max_length = cur_length;
            }
        }

        return max_length;
    }
};