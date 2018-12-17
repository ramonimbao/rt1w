use rand::Rng;
use serde_json::Value;

pub fn get_f64_or_rand(item: &Value) -> Option<f64> {
    match (item.is_number(), item.is_string()) {
        (true, false) => item.as_f64(),
        (false, true) => {
            let mut nums: Vec<f64> = item
                .as_str()
                .unwrap_or("0,1")
                .split(',')
                .map(|n| n.replace(" ", "").parse::<f64>().unwrap_or(0.0))
                .take(2)
                .collect();
            if nums.len() < 2 {
                nums.push(0.0);
            }
            if (nums[0] - nums[1]).abs() <= std::f64::EPSILON {
                return Some(nums[0]);
            }

            if nums[0] > nums[1] {
                nums.swap(0, 1);
            }

            let mut rng = rand::thread_rng();
            Some(rng.gen_range(nums[0], nums[1]))
        }
        (_, _) => None,
    }
}
