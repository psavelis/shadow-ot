'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { ArrowLeft, Send, AlertCircle, X } from 'lucide-react'

const categories = [
  { id: 'general', name: 'General Discussion', description: 'Talk about anything related to Shadow OT' },
  { id: 'guides', name: 'Guides & Tutorials', description: 'Share your knowledge with the community' },
  { id: 'trading', name: 'Trading', description: 'Buy, sell, and trade items' },
  { id: 'support', name: 'Support', description: 'Get help with technical issues' },
  { id: 'creative', name: 'Creative Corner', description: 'Fan art, stories, and videos' },
]

const realmCategories = [
  { id: 'shadowveil', name: 'Shadowveil', color: 'text-red-400' },
  { id: 'aetheria', name: 'Aetheria', color: 'text-blue-400' },
  { id: 'warbound', name: 'Warbound', color: 'text-orange-400' },
]

export default function NewThreadPage() {
  const [title, setTitle] = useState('')
  const [category, setCategory] = useState('')
  const [content, setContent] = useState('')
  const [tags, setTags] = useState<string[]>([])
  const [tagInput, setTagInput] = useState('')
  const [errors, setErrors] = useState<{ title?: string; category?: string; content?: string }>({})
  const [isSubmitting, setIsSubmitting] = useState(false)

  const addTag = () => {
    const tag = tagInput.trim().toLowerCase()
    if (tag && !tags.includes(tag) && tags.length < 5) {
      setTags([...tags, tag])
      setTagInput('')
    }
  }

  const removeTag = (tagToRemove: string) => {
    setTags(tags.filter(t => t !== tagToRemove))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    const newErrors: typeof errors = {}

    if (title.trim().length < 5) {
      newErrors.title = 'Title must be at least 5 characters'
    }
    if (!category) {
      newErrors.category = 'Please select a category'
    }
    if (content.trim().length < 20) {
      newErrors.content = 'Content must be at least 20 characters'
    }

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors)
      return
    }

    setErrors({})
    setIsSubmitting(true)
    
    // Simulate API call
    setTimeout(() => {
      window.location.href = '/forum'
    }, 1000)
  }

  return (
    <div className="min-h-screen bg-shadow-950 py-8">
      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
          <Link href="/forum" className="inline-flex items-center gap-2 text-shadow-400 hover:text-white mb-6 transition-colors">
            <ArrowLeft className="w-4 h-4" />
            Back to Forums
          </Link>

          <h1 className="text-2xl font-display font-bold mb-6">Create New Thread</h1>

          <form onSubmit={handleSubmit} className="space-y-6">
            {/* Title */}
            <div className="card">
              <label className="block text-sm font-medium text-shadow-300 mb-2">
                Thread Title *
              </label>
              <input
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="Enter a descriptive title..."
                className={`w-full px-4 py-3 bg-shadow-800 border rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 ${
                  errors.title 
                    ? 'border-red-500 focus:ring-red-500/50' 
                    : 'border-shadow-600 focus:ring-accent-500/50'
                }`}
              />
              {errors.title && (
                <p className="mt-2 text-sm text-red-400 flex items-center gap-1">
                  <AlertCircle className="w-4 h-4" />
                  {errors.title}
                </p>
              )}
            </div>

            {/* Category */}
            <div className="card">
              <label className="block text-sm font-medium text-shadow-300 mb-2">
                Category *
              </label>
              <div className="grid sm:grid-cols-2 gap-3">
                {categories.map((cat) => (
                  <button
                    key={cat.id}
                    type="button"
                    onClick={() => setCategory(cat.id)}
                    className={`p-3 rounded-lg border text-left transition-all ${
                      category === cat.id
                        ? 'border-accent-500 bg-accent-500/10'
                        : 'border-shadow-700 hover:border-shadow-500'
                    }`}
                  >
                    <p className="font-medium text-white">{cat.name}</p>
                    <p className="text-shadow-500 text-xs mt-1">{cat.description}</p>
                  </button>
                ))}
              </div>
              <div className="mt-4">
                <p className="text-sm text-shadow-400 mb-2">Or choose a realm forum:</p>
                <div className="flex gap-2">
                  {realmCategories.map((realm) => (
                    <button
                      key={realm.id}
                      type="button"
                      onClick={() => setCategory(realm.id)}
                      className={`px-3 py-1.5 rounded-lg border text-sm transition-all ${
                        category === realm.id
                          ? 'border-accent-500 bg-accent-500/10'
                          : 'border-shadow-700 hover:border-shadow-500'
                      } ${realm.color}`}
                    >
                      {realm.name}
                    </button>
                  ))}
                </div>
              </div>
              {errors.category && (
                <p className="mt-2 text-sm text-red-400 flex items-center gap-1">
                  <AlertCircle className="w-4 h-4" />
                  {errors.category}
                </p>
              )}
            </div>

            {/* Content */}
            <div className="card">
              <label className="block text-sm font-medium text-shadow-300 mb-2">
                Content *
              </label>
              <textarea
                value={content}
                onChange={(e) => setContent(e.target.value)}
                placeholder="Write your thread content here... Markdown is supported."
                rows={12}
                className={`w-full px-4 py-3 bg-shadow-800 border rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 resize-y ${
                  errors.content 
                    ? 'border-red-500 focus:ring-red-500/50' 
                    : 'border-shadow-600 focus:ring-accent-500/50'
                }`}
              />
              {errors.content && (
                <p className="mt-2 text-sm text-red-400 flex items-center gap-1">
                  <AlertCircle className="w-4 h-4" />
                  {errors.content}
                </p>
              )}
              <div className="flex justify-between mt-2">
                <p className="text-shadow-500 text-sm">Markdown supported</p>
                <p className={`text-sm ${content.length < 20 ? 'text-shadow-500' : 'text-green-400'}`}>
                  {content.length} characters
                </p>
              </div>
            </div>

            {/* Tags */}
            <div className="card">
              <label className="block text-sm font-medium text-shadow-300 mb-2">
                Tags (optional, max 5)
              </label>
              <div className="flex flex-wrap gap-2 mb-3">
                {tags.map((tag) => (
                  <span key={tag} className="flex items-center gap-1 px-2 py-1 bg-shadow-700 text-shadow-300 rounded text-sm">
                    #{tag}
                    <button type="button" onClick={() => removeTag(tag)} className="text-shadow-500 hover:text-white">
                      <X className="w-3 h-3" />
                    </button>
                  </span>
                ))}
              </div>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={tagInput}
                  onChange={(e) => setTagInput(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && (e.preventDefault(), addTag())}
                  placeholder="Add a tag..."
                  disabled={tags.length >= 5}
                  className="flex-1 px-4 py-2 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50 disabled:opacity-50"
                />
                <button
                  type="button"
                  onClick={addTag}
                  disabled={tags.length >= 5 || !tagInput.trim()}
                  className="btn-secondary disabled:opacity-50"
                >
                  Add
                </button>
              </div>
            </div>

            {/* Submit */}
            <div className="flex justify-end gap-4">
              <Link href="/forum" className="btn-secondary">
                Cancel
              </Link>
              <button type="submit" disabled={isSubmitting} className="btn-primary flex items-center gap-2">
                <Send className="w-4 h-4" />
                {isSubmitting ? 'Creating...' : 'Create Thread'}
              </button>
            </div>
          </form>
        </motion.div>
      </div>
    </div>
  )
}


