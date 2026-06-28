/**
 * MidStream Agent - High-level wrapper for Lean Agentic Learning System
 */

export interface AgentConfig {
  maxHistory?: number;
  embeddingDim?: number;
  schedulingPolicy?: string;
}

export interface AnalysisResult {
  messageCount: number;
  patterns: any[];
  metaLearning: any;
  temporalAnalysis?: any;
}

export interface BehaviorAnalysis {
  attractorType?: string;
  lyapunovExponent?: number;
  isStable?: boolean;
  isChaotic?: boolean;
}

export class MidStreamAgent {
  private wasmAgent: any;
  private config: AgentConfig;
  private conversationHistory: string[] = [];
  private rewardHistory: number[] = [];

  constructor(config: AgentConfig = {}) {
    this.config = {
      maxHistory: config.maxHistory || 1000,
      embeddingDim: config.embeddingDim || 3,
      schedulingPolicy: config.schedulingPolicy || 'EDF',
    };

    // Load WASM module
    try {
      const wasm = require('../wasm/midstream_wasm');
      this.wasmAgent = new wasm.MidStreamAgent(this.config);
    } catch (error) {
      console.warn('WASM module not available, using fallback implementation');
      this.wasmAgent = null;
    }
  }

  /**
   * Process a single message
   */
  processMessage(message: string): any {
    this.conversationHistory.push(message);

    if (this.conversationHistory.length > this.config.maxHistory!) {
      this.conversationHistory.shift();
    }

    if (this.wasmAgent) {
      return this.wasmAgent.process_message(message);
    }

    // Fallback implementation
    return {
      processed: true,
      message,
      timestamp: Date.now(),
    };
  }

  /**
   * Analyze a complete conversation
   */
  analyzeConversation(messages: string[]): AnalysisResult {
    if (this.wasmAgent) {
      return this.wasmAgent.analyze_conversation(messages);
    }

    // Fallback implementation
    return {
      messageCount: messages.length,
      patterns: [],
      metaLearning: {
        currentLevel: 'Object',
        knowledgeCounts: [messages.length, 0, 0, 0],
      },
    };
  }

  /**
   * Compare two sequences using temporal analysis
   */
  compareSequences(seq1: string[], seq2: string[], algorithm: string = 'dtw'): number {
    if (this.wasmAgent) {
      const comparator = this.wasmAgent.temporal;
      return comparator?.compare(seq1, seq2, algorithm) || 0;
    }

    // Fallback implementations used when the WASM module is unavailable.
    // Two empty sequences are identical; an empty vs non-empty pair shares
    // nothing. Guarding here also avoids the 0/0 = NaN case below.
    if (seq1.length === 0 && seq2.length === 0) {
      return 1;
    }
    if (seq1.length === 0 || seq2.length === 0) {
      return 0;
    }

    if (algorithm === 'lcs') {
      // Longest-common-subsequence similarity, normalised by the longer
      // sequence so a shared ordered prefix/suffix scores highly.
      const lcs = this.longestCommonSubsequenceLength(seq1, seq2);
      return lcs / Math.max(seq1.length, seq2.length);
    }

    // Default fallback: Jaccard set similarity.
    const set1 = new Set(seq1);
    const set2 = new Set(seq2);
    const intersection = new Set([...set1].filter(x => set2.has(x)));
    const union = new Set([...set1, ...set2]);

    return intersection.size / union.size;
  }

  /**
   * Length of the longest common subsequence of two sequences (fallback
   * helper for {@link compareSequences} when the WASM comparator is absent).
   */
  private longestCommonSubsequenceLength(a: string[], b: string[]): number {
    const m = a.length;
    const n = b.length;
    const dp: number[][] = Array.from({ length: m + 1 }, () =>
      new Array<number>(n + 1).fill(0)
    );
    for (let i = 1; i <= m; i++) {
      for (let j = 1; j <= n; j++) {
        dp[i][j] = a[i - 1] === b[j - 1]
          ? dp[i - 1][j - 1] + 1
          : Math.max(dp[i - 1][j], dp[i][j - 1]);
      }
    }
    return dp[m][n];
  }

  /**
   * Detect pattern in sequence
   */
  detectPattern(sequence: string[], pattern: string[]): number[] {
    const positions: number[] = [];

    if (pattern.length === 0 || sequence.length < pattern.length) {
      return positions;
    }

    for (let i = 0; i <= sequence.length - pattern.length; i++) {
      let match = true;
      for (let j = 0; j < pattern.length; j++) {
        if (sequence[i + j] !== pattern[j]) {
          match = false;
          break;
        }
      }
      if (match) {
        positions.push(i);
      }
    }

    return positions;
  }

  /**
   * Analyze behavior using attractor analysis
   */
  analyzeBehavior(rewards: number[]): BehaviorAnalysis {
    this.rewardHistory.push(...rewards);

    if (this.rewardHistory.length > this.config.maxHistory!) {
      this.rewardHistory = this.rewardHistory.slice(-this.config.maxHistory!);
    }

    // With no samples there is no behaviour to analyse — report a neutral,
    // stable result rather than propagating NaN from a 0/0 mean.
    if (rewards.length === 0) {
      return { isStable: true, isChaotic: false, lyapunovExponent: 0 };
    }

    // Simple stability check (fallback). Rewards are expected in the [0, 1]
    // range, where a uniformly random ("chaotic") stream has a standard
    // deviation of ~0.29; a steady stream sits well under 0.1. The chaotic
    // threshold (0.12) sits comfortably above the stable band (~0.015) yet
    // far enough below the random mean to keep the unseeded-random test
    // non-flaky (P(stdDev < 0.12) ~ 0.02% for n=20).
    const mean = rewards.reduce((a, b) => a + b, 0) / rewards.length;
    const variance = rewards.reduce((sum, r) => sum + Math.pow(r - mean, 2), 0) / rewards.length;
    const stdDev = Math.sqrt(variance);

    return {
      isStable: stdDev < 0.1,
      isChaotic: stdDev > 0.12,
      lyapunovExponent: stdDev > 0.12 ? 0.5 : -0.5,
    };
  }

  /**
   * Perform meta-learning
   */
  learn(content: string, reward: number): void {
    this.rewardHistory.push(reward);

    if (this.wasmAgent) {
      this.wasmAgent.process_message(content);
    }
  }

  /**
   * Get meta-learning summary
   */
  getMetaLearningSummary(): any {
    if (this.wasmAgent) {
      return this.wasmAgent.get_status();
    }

    return {
      currentLevel: 'Object',
      knowledgeCounts: [this.conversationHistory.length, 0, 0, 0],
      numStrangeLoops: 0,
      numModificationRules: 0,
      safetyViolations: 0,
    };
  }

  /**
   * Get agent status
   */
  getStatus(): any {
    return {
      conversationHistorySize: this.conversationHistory.length,
      rewardHistorySize: this.rewardHistory.length,
      config: this.config,
      metaLearning: this.getMetaLearningSummary(),
      averageReward: this.rewardHistory.length > 0
        ? this.rewardHistory.reduce((a, b) => a + b, 0) / this.rewardHistory.length
        : 0,
    };
  }

  /**
   * Clear all history
   */
  reset(): void {
    this.conversationHistory = [];
    this.rewardHistory = [];
  }
}
