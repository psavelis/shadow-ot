/**
 * Validation utilities for Shadow OT forms
 */

// Character name validation
export const CHARACTER_NAME_REGEX = /^[A-Za-z][A-Za-z ']{1,28}[A-Za-z]$/
export const CHARACTER_NAME_MIN = 3
export const CHARACTER_NAME_MAX = 30

export function validateCharacterName(name: string): { valid: boolean; error?: string } {
  if (!name) {
    return { valid: false, error: 'Name is required' }
  }
  
  if (name.length < CHARACTER_NAME_MIN) {
    return { valid: false, error: `Name must be at least ${CHARACTER_NAME_MIN} characters` }
  }
  
  if (name.length > CHARACTER_NAME_MAX) {
    return { valid: false, error: `Name cannot exceed ${CHARACTER_NAME_MAX} characters` }
  }
  
  if (!CHARACTER_NAME_REGEX.test(name)) {
    return { 
      valid: false, 
      error: 'Name must start and end with a letter, and can only contain letters, spaces, and apostrophes' 
    }
  }

  // Check for consecutive spaces
  if (/\s{2,}/.test(name)) {
    return { valid: false, error: 'Name cannot contain consecutive spaces' }
  }

  // Check for words (each word must be at least 2 characters)
  const words = name.split(/\s+/)
  for (const word of words) {
    if (word.length < 2) {
      return { valid: false, error: 'Each word must be at least 2 characters' }
    }
  }

  return { valid: true }
}

// Email validation
export const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

export function validateEmail(email: string): { valid: boolean; error?: string } {
  if (!email) {
    return { valid: false, error: 'Email is required' }
  }
  
  if (!EMAIL_REGEX.test(email)) {
    return { valid: false, error: 'Invalid email address' }
  }

  return { valid: true }
}

// Password validation
export const PASSWORD_MIN = 8
export const PASSWORD_MAX = 128

export interface PasswordStrength {
  score: number // 0-4
  label: 'weak' | 'fair' | 'good' | 'strong' | 'very strong'
  color: string
  suggestions: string[]
}

export function validatePassword(password: string): { valid: boolean; error?: string } {
  if (!password) {
    return { valid: false, error: 'Password is required' }
  }
  
  if (password.length < PASSWORD_MIN) {
    return { valid: false, error: `Password must be at least ${PASSWORD_MIN} characters` }
  }
  
  if (password.length > PASSWORD_MAX) {
    return { valid: false, error: `Password cannot exceed ${PASSWORD_MAX} characters` }
  }

  return { valid: true }
}

export function getPasswordStrength(password: string): PasswordStrength {
  let score = 0
  const suggestions: string[] = []

  if (password.length >= 8) score++
  else suggestions.push('Use at least 8 characters')
  
  if (password.length >= 12) score++
  
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++
  else suggestions.push('Include uppercase and lowercase letters')
  
  if (/\d/.test(password)) score++
  else suggestions.push('Include numbers')
  
  if (/[^a-zA-Z0-9]/.test(password)) score++
  else suggestions.push('Include special characters')

  // Penalize common patterns
  if (/^[a-zA-Z]+$/.test(password) || /^\d+$/.test(password)) {
    score = Math.max(0, score - 1)
  }

  const labels: PasswordStrength['label'][] = ['weak', 'fair', 'good', 'strong', 'very strong']
  const colors = ['bg-red-500', 'bg-orange-500', 'bg-yellow-500', 'bg-green-400', 'bg-green-500']

  return {
    score: Math.min(score, 4),
    label: labels[Math.min(score, 4)],
    color: colors[Math.min(score, 4)],
    suggestions,
  }
}

// Username validation
export const USERNAME_REGEX = /^[a-zA-Z0-9_]{3,20}$/

export function validateUsername(username: string): { valid: boolean; error?: string } {
  if (!username) {
    return { valid: false, error: 'Username is required' }
  }
  
  if (username.length < 3) {
    return { valid: false, error: 'Username must be at least 3 characters' }
  }
  
  if (username.length > 20) {
    return { valid: false, error: 'Username cannot exceed 20 characters' }
  }
  
  if (!USERNAME_REGEX.test(username)) {
    return { valid: false, error: 'Username can only contain letters, numbers, and underscores' }
  }

  return { valid: true }
}

// Guild name validation
export const GUILD_NAME_REGEX = /^[A-Za-z][A-Za-z0-9 ]{2,28}[A-Za-z0-9]$/

export function validateGuildName(name: string): { valid: boolean; error?: string } {
  if (!name) {
    return { valid: false, error: 'Guild name is required' }
  }
  
  if (name.length < 4) {
    return { valid: false, error: 'Guild name must be at least 4 characters' }
  }
  
  if (name.length > 30) {
    return { valid: false, error: 'Guild name cannot exceed 30 characters' }
  }
  
  if (!GUILD_NAME_REGEX.test(name)) {
    return { valid: false, error: 'Guild name must start with a letter and can only contain letters, numbers, and spaces' }
  }

  return { valid: true }
}

// Forum content validation
export function validateForumContent(content: string, minLength = 10, maxLength = 50000): { valid: boolean; error?: string } {
  if (!content) {
    return { valid: false, error: 'Content is required' }
  }
  
  const trimmed = content.trim()
  
  if (trimmed.length < minLength) {
    return { valid: false, error: `Content must be at least ${minLength} characters` }
  }
  
  if (trimmed.length > maxLength) {
    return { valid: false, error: `Content cannot exceed ${maxLength} characters` }
  }

  return { valid: true }
}

// Thread title validation
export function validateThreadTitle(title: string): { valid: boolean; error?: string } {
  if (!title) {
    return { valid: false, error: 'Title is required' }
  }
  
  const trimmed = title.trim()
  
  if (trimmed.length < 5) {
    return { valid: false, error: 'Title must be at least 5 characters' }
  }
  
  if (trimmed.length > 100) {
    return { valid: false, error: 'Title cannot exceed 100 characters' }
  }

  return { valid: true }
}

// Generic required field
export function validateRequired(value: unknown, fieldName = 'Field'): { valid: boolean; error?: string } {
  if (value === null || value === undefined || value === '') {
    return { valid: false, error: `${fieldName} is required` }
  }
  return { valid: true }
}

// Combine multiple validations
export function combineValidations(
  ...validations: Array<{ valid: boolean; error?: string }>
): { valid: boolean; errors: string[] } {
  const errors: string[] = []
  
  for (const validation of validations) {
    if (!validation.valid && validation.error) {
      errors.push(validation.error)
    }
  }

  return {
    valid: errors.length === 0,
    errors,
  }
}

