import {describe, expect, test, it} from '@jest/globals';
import { PersonBuilder } from './builder';

describe('PersonBuilder tests', () => {
    it.each(
        [["Dominik", 32]]
    )("build person with name %p and age %p", (name: string, age: number) => {
        const builder = new PersonBuilder().withName(name).withAge(age);
        
        const person = builder.build()
        expect(person.name).toBe(name)
        expect(person.age).toBe(age)
    })
});