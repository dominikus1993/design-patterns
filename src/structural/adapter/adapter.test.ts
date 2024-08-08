import {describe, expect, test, it} from '@jest/globals';
import { Client, ConstructorProvider, InMemoryProviderAdapter } from './adapter';

describe('PersonBuilder tests', () => {
    it.each(
        [[32, 32]]
    )("build person with name %p and age %p", (num: number, expected: number) => {
        const provider = new ConstructorProvider(num);

        const adapter = new  InMemoryProviderAdapter(provider);
        
        const client = new Client(adapter);
        const sum = client.sum();
        expect(sum).toBe(expected)
    })
});