initSidebarItems({"enum":[["Error","The errors that might occur in the this crate and solution-type."]],"fn":[["is_score_better","Compares two sets of election scores based on desirability and returns true if `this` is better than `that`."],["setup_inputs","Converts raw inputs to types used in this crate."],["to_support_map","Build the support map from the assignments."],["to_supports","Same as [`to_support_map`] except it returns a flat vector."]],"macro":[["generate_solution_type","Generates a struct to store the election result in a small/compact way. This can encode a structure which is the equivalent of a `sp_npos_elections::Assignment<_>`."]],"mod":[["balancing","Balancing algorithm implementation."],["helpers","Helper methods for npos-elections."],["node","(very) Basic implementation of a graph node used in the reduce algorithm."],["phragmen","Implementation of the sequential-phragmen election method."],["phragmms","Implementation of the PhragMMS method."],["pjr","Implements functions and interfaces to check solutions for being t-PJR."],["reduce","Rust implementation of the Phragmén reduce algorithm. This can be used by any off chain application to reduce cycles from the edge assignment, which will result in smaller size."],["traits","Traits for the npos-election operations."]],"struct":[["Assignment","A voter’s stake assignment among a set of targets, represented as ratios."],["Candidate","A candidate entity for the election."],["Edge","A vote being casted by a [`Voter`] to a [`Candidate`] is an `Edge`."],["ElectionResult","Final result of the election."],["IndexAssignment","The [`IndexAssignment`] type is an intermediate between the assignments list ([`&[Assignment<T>]`][Assignment]) and `SolutionOf<T>`."],["StakedAssignment","A voter’s stake assignment among a set of targets, represented as absolute values in the scale of [`ExtendedBalance`]."],["Support","A structure to demonstrate the election result from the perspective of the candidate, i.e. how much support each candidate is receiving."],["Voter","A voter entity."]],"trait":[["EvaluateSupport","Extension trait for evaluating a support map or vector."]],"type":[["CandidatePtr","A pointer to a candidate struct with interior mutability."],["ElectionScore","The score of an assignment. This can be computed from the support map via [`EvaluateSupport::evaluate`]."],["ExtendedBalance","A type in which performing operations on vote weights are safe."],["IndexAssignmentOf","A type alias for [`IndexAssignment`] made from [`crate::Solution`]."],["SupportMap","Linkage from a winner to their [`Support`]."],["Supports","A target-major representation of the the election outcome."],["VoteWeight","A type which is used in the API of this crate as a numeric weight of a vote, most often the stake of the voter. It is always converted to [`ExtendedBalance`] for computation."]]});