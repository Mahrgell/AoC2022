use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Candidate<BaseType: Clone, Cost: Clone> {
    pub cand: BaseType,
    pub cost: Cost,
    pub total_guess: Cost,
}

pub enum AStarGoal<StaticData, BaseType: Eq> {
    Func(fn(&BaseType, &StaticData) -> bool),
    Fixed(BaseType),
}

pub struct AStarSolver<StaticData, BaseType, Cost>
where
    BaseType: Clone + Eq + std::hash::Hash,
    Cost: Clone,
{
    static_data: StaticData,
    fn_next_candidates: fn(&BaseType, &StaticData, &Cost) -> Vec<Candidate<BaseType, Cost>>,
    goal: AStarGoal<StaticData, BaseType>,
    fn_compare: fn(&Cost, &Cost) -> Ordering,
    done: HashMap<BaseType, (Option<BaseType>, Cost)>,
    todo: Vec<(Candidate<BaseType, Cost>, Option<BaseType>)>,
}

impl<StaticData, BaseType: Eq> AStarGoal<StaticData, BaseType> {
    fn reached(&self, bt: &BaseType, sd: &StaticData) -> bool {
        match self {
            AStarGoal::Func(f) => f(bt, sd),
            AStarGoal::Fixed(val) => *val == *bt
        }
    }
}

pub struct AStarResult<BaseType, Cost>
where
    BaseType: Clone + Eq + std::hash::Hash,
    Cost: Clone,
{
    result: HashMap<BaseType, (Option<BaseType>, Cost)>,
}

impl<BaseType: Clone, Cost: Clone+Default> Candidate<BaseType, Cost> {
    pub fn start(start: BaseType) -> Candidate<BaseType, Cost>
     {
        Candidate { cand: start, cost: Cost::default(), total_guess: Cost::default() }
     }
} 

impl<StaticData, BaseType, Cost> AStarSolver<StaticData, BaseType, Cost>
where
    BaseType: Clone + Eq + std::hash::Hash,
    Cost: Clone,
{
    pub fn new_with_compare(
        static_data: StaticData,
        initial_candidates: Vec<Candidate<BaseType, Cost>>,
        fn_next_candidates: fn(&BaseType, &StaticData, &Cost) -> Vec<Candidate<BaseType, Cost>>,
        goal: AStarGoal<StaticData, BaseType>,
        fn_compare: fn(&Cost, &Cost) -> Ordering,
    ) -> AStarSolver<StaticData, BaseType, Cost> {
        let todo = initial_candidates
            .iter()
            .map(|cand| (cand.clone(), None))
            .collect();
        AStarSolver {
            static_data,
            fn_next_candidates,
            goal,
            fn_compare,
            done: HashMap::new(),
            todo,
        }
    }

    pub fn solve(mut self) -> (AStarResult<BaseType, Cost>, Option<BaseType>) {
        let mut final_cand = None;
        while let Some((cand, prev)) = self.todo.pop() {
            if self.done.contains_key(&cand.cand) {
                continue;
            }
            self.done
                .insert(cand.cand.clone(), (prev, cand.cost.clone()));
            if self.goal.reached(&cand.cand, &self.static_data) {             
                final_cand = Some(cand.cand);
                break;
            }
            for next_cand in (self.fn_next_candidates)(&cand.cand, &self.static_data, &cand.cost) {
                if self.done.contains_key(&next_cand.cand) {
                    continue;
                }
                self.todo.push((next_cand, Some(cand.cand.clone())));
            }
            self.todo
                .sort_by(|a, b| (self.fn_compare)(&b.0.total_guess, &a.0.total_guess));
        }
        (AStarResult{result: self.done}, final_cand)
    }
}

impl<BaseType, Cost> AStarResult<BaseType, Cost>
where
    BaseType: Clone + Eq + std::hash::Hash,
    Cost: Clone,
    {
        pub fn get_cost(&self, bt: &BaseType) -> Option<Cost> {
            if let Some((_prev, cost)) = self.result.get(bt) {
                return Some(cost.clone());
            }
            None
        }
    
        pub fn get_path(&self, bt: &BaseType) -> Vec<BaseType> {
            if !self.result.contains_key(bt) {
                return Vec::new();
            }
            let mut current = bt.clone();
            let mut path = Vec::new();
            while let Some(prev) = &self.result[&current].0 {
                path.push(current);
                current = prev.clone();
            }
            path.push(current);
            path.reverse();
            path
        }
    }

impl<StaticData, BaseType, Cost> AStarSolver<StaticData, BaseType, Cost>
where
    BaseType: Clone + Eq + std::hash::Hash,
    Cost: Clone + Ord,
{
    pub fn new(
        static_data: StaticData,
        initial_candidates: Vec<Candidate<BaseType, Cost>>,
        fn_next_candidates: fn(&BaseType, &StaticData, &Cost) -> Vec<Candidate<BaseType, Cost>>,
        goal: AStarGoal<StaticData, BaseType>,
    ) -> AStarSolver<StaticData, BaseType, Cost> {
        AStarSolver::new_with_compare(
            static_data,
            initial_candidates,
            fn_next_candidates,
            goal,
            |a, b| a.cmp(b),
        )
    }
}