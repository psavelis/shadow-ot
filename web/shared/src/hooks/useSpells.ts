'use client'

import { useQuery } from '@tanstack/react-query'
import { spellApi } from '../api/endpoints'
import type { Spell, SpellElement, SpellType, Vocation } from '../types'

export function useSpells(params?: {
  element?: SpellElement
  type?: SpellType
  vocation?: Vocation
  premium?: boolean
  search?: string
}) {
  return useQuery({
    queryKey: ['spells', params],
    queryFn: () => spellApi.getAll(params),
    staleTime: 1000 * 60 * 30, // 30 minutes - spells don't change often
  })
}

export function useSpell(id: string) {
  return useQuery({
    queryKey: ['spell', id],
    queryFn: () => spellApi.getById(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 60, // 1 hour
  })
}

export function useSpellByWords(words: string) {
  return useQuery({
    queryKey: ['spell', 'words', words],
    queryFn: () => spellApi.getByWords(words),
    enabled: !!words,
    staleTime: 1000 * 60 * 60,
  })
}

export function useSpellsByVocation(vocation: Vocation) {
  return useQuery({
    queryKey: ['spells', 'vocation', vocation],
    queryFn: () => spellApi.getByVocation(vocation),
    enabled: !!vocation,
    staleTime: 1000 * 60 * 30,
  })
}

export function useSpellsByElement(element: SpellElement) {
  return useQuery({
    queryKey: ['spells', 'element', element],
    queryFn: () => spellApi.getByElement(element),
    enabled: !!element,
    staleTime: 1000 * 60 * 30,
  })
}

export function useRunes() {
  return useQuery({
    queryKey: ['spells', 'runes'],
    queryFn: () => spellApi.getRunes(),
    staleTime: 1000 * 60 * 30,
  })
}


