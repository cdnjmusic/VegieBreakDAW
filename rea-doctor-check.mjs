#!/usr/bin/env node
import { execSync } from 'child_process';

console.log('Running REA Ghidra doctor check...');
try {
  const output = execSync('node scripts/rea.mjs doctor --provider ghidra --json', {
    cwd: 'C:\\Users\\Potaku\\Documents\\REA_FL\\rea',
    encoding: 'utf8',
    maxBuffer: 1024 * 1024,
  });
  console.log(output);
} catch (e) {
  console.error('Doctor check failed or returned non-zero:', e.message);
  console.error(e.stdout || '');
  process.exit(1);
}
