import {describe, expect, test} from '@jest/globals';
import { TestAdd, TestSub } from './factoryMethod';

describe('creator pattern module', () => {
  test('adds 1 + 2 to equal 3', () => {
    const add = new TestAdd();
    expect(add.doOperation(1, 2)).toBe(3);
  });

  test('sub 1 + 2 to equal -1', () => {
    const sub = new TestSub();
    expect(sub.doOperation(1, 2)).toBe(-1);
  });
});