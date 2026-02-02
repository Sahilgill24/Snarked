use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vm::{Cpu, TraceRow};

/// Represents the execution trace of a program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    pub rows: Vec<TraceRow>,
    pub num_rows: usize,
}

impl ExecutionTrace {
    /// Create a new execution trace from a CPU after running
    pub fn from_cpu(cpu: &Cpu) -> Self {
        let rows = cpu.get_trace();
        let num_rows = rows.len();
        ExecutionTrace { rows, num_rows }
    }

    /// Get the trace as a table for display
    pub fn to_table(&self) -> Vec<HashMap<String, String>> {
        self.rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut map = HashMap::new();
                map.insert("cycle".to_string(), i.to_string());
                map.insert("pc".to_string(), row.pc.to_string());
                map.insert("instruction".to_string(), format!("0x{:04X}", row.instruction));
                for (j, &reg_val) in row.registers.iter().enumerate() {
                    map.insert(format!("r{}", j), reg_val.to_string());
                }
                map
            })
            .collect()
    }

    /// Verify trace constraints (simple checks)
    pub fn verify_constraints(&self) -> bool {
        // Check that trace is not empty
        if self.rows.is_empty() {
            return false;
        }

        // Check that all register values are valid (u16)
        for row in &self.rows {
            for &reg_val in &row.registers {
                if reg_val > u16::MAX {
                    return false;
                }
            }
        }

        true
    }

    /// Get a polynomial representation of a register's trace
    pub fn get_register_trace(&self, reg_idx: usize) -> Vec<u16> {
        if reg_idx >= 8 {
            return Vec::new();
        }
        self.rows.iter().map(|row| row.registers[reg_idx]).collect()
    }

    /// Get program counter trace
    pub fn get_pc_trace(&self) -> Vec<u32> {
        self.rows.iter().map(|row| row.pc).collect()
    }

    /// Get instruction trace
    pub fn get_instruction_trace(&self) -> Vec<u16> {
        self.rows.iter().map(|row| row.instruction).collect()
    }
}

/// Constraint polynomials for the trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintPolynomials {
    /// Polynomials encoding register constraints
    pub register_constraints: Vec<Vec<u64>>,
    /// Polynomial encoding instruction validity
    pub instruction_constraints: Vec<u64>,
    /// Polynomial encoding program counter transitions
    pub pc_constraints: Vec<u64>,
}

impl ConstraintPolynomials {
    /// Generate constraints from a trace
    pub fn from_trace(trace: &ExecutionTrace) -> Self {
        let mut register_constraints = Vec::new();

        // Generate constraints for each register
        for reg_idx in 0..8 {
            let reg_trace = trace.get_register_trace(reg_idx);
            let constraints = Self::generate_register_constraints(&reg_trace);
            register_constraints.push(constraints);
        }

        // Generate instruction constraints
        let instruction_trace = trace.get_instruction_trace();
        let instruction_constraints = Self::generate_instruction_constraints(&instruction_trace);

        // Generate PC constraints
        let pc_trace = trace.get_pc_trace();
        let pc_constraints = Self::generate_pc_constraints(&pc_trace);

        ConstraintPolynomials {
            register_constraints,
            instruction_constraints,
            pc_constraints,
        }
    }

    fn generate_register_constraints(trace: &[u16]) -> Vec<u64> {
        trace.iter().map(|&v| v as u64).collect()
    }

    fn generate_instruction_constraints(trace: &[u16]) -> Vec<u64> {
        trace.iter().map(|&v| (v as u64).wrapping_mul(v as u64)).collect()
    }

    fn generate_pc_constraints(trace: &[u32]) -> Vec<u64> {
        trace.iter().map(|&v| v as u64).collect()
    }

    /// Evaluate all constraints at a given point (simplified for demo)
    pub fn evaluate_at(&self, x: u64) -> u64 {
        let mut result = 0u64;

        for reg_const in &self.register_constraints {
            for (i, &coeff) in reg_const.iter().enumerate() {
                result = result.wrapping_add(coeff.wrapping_mul(x.wrapping_pow(i as u32)));
            }
        }

        for (i, &coeff) in self.instruction_constraints.iter().enumerate() {
            result = result.wrapping_add(coeff.wrapping_mul(x.wrapping_pow(i as u32)));
        }

        for (i, &coeff) in self.pc_constraints.iter().enumerate() {
            result = result.wrapping_add(coeff.wrapping_mul(x.wrapping_pow(i as u32)));
        }

        result
    }
}

/// Proof structure (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceProof {
    pub trace_hash: String,
    pub constraints_hash: String,
    pub evaluation_points: Vec<u64>,
    pub evaluations: Vec<u64>,
}

impl TraceProof {
    /// Generate a proof from a trace
    pub fn generate(trace: &ExecutionTrace) -> Self {
        let trace_hash = Self::hash_data(
            &trace
                .rows
                .iter()
                .flat_map(|row| {
                    row.registers
                        .iter()
                        .map(|&r| r as u8)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );

        let constraints = ConstraintPolynomials::from_trace(trace);
        let constraints_data = format!("{:?}", constraints);
        let constraints_hash = Self::hash_data(constraints_data.as_bytes());

        // Generate evaluation points (random-like, deterministic for reproducibility)
        let num_evaluations = (trace.num_rows as f64).sqrt().ceil() as usize;
        let mut evaluation_points = Vec::new();
        for i in 0..num_evaluations {
            evaluation_points.push((i as u64).wrapping_mul(7).wrapping_add(3));
        }

        // Evaluate constraints at these points
        let mut evaluations = Vec::new();
        for &point in &evaluation_points {
            evaluations.push(constraints.evaluate_at(point));
        }

        TraceProof {
            trace_hash,
            constraints_hash,
            evaluation_points,
            evaluations,
        }
    }

    fn hash_data(data: &[u8]) -> String {
        let mut hash = 0u64;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    }

    /// Verify the proof (simplified - just checks structure)
    pub fn verify(&self) -> bool {
        self.evaluation_points.len() == self.evaluations.len()
            && !self.trace_hash.is_empty()
            && !self.constraints_hash.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_trace_creation() {
        let program = vec![
            0x1100, // MOV r1, 0
            0x1200, // MOV r2, 0
        ];
        let mut cpu = Cpu::new(program);
        cpu.run();

        let trace = ExecutionTrace::from_cpu(&cpu);
        assert!(!trace.rows.is_empty());
        assert!(trace.verify_constraints());
    }

    #[test]
    fn test_trace_proof_generation() {
        let program = vec![
            0x3102, // ADD r1, r0, r2
            0x3213, // ADD r2, r1, r3
        ];
        let mut cpu = Cpu::new(program);
        cpu.run();

        let trace = ExecutionTrace::from_cpu(&cpu);
        let proof = TraceProof::generate(&trace);

        assert!(proof.verify());
        assert_eq!(proof.evaluation_points.len(), proof.evaluations.len());
    }
}

// Trace format enhanced Feb 2026
// Added step-by-step execution recording
