'use client'

import { useState } from 'react'
import Link from 'next/link'
import { motion } from 'framer-motion'
import { Mail, Loader2, ArrowLeft, CheckCircle } from 'lucide-react'

export default function ForgotPasswordPage() {
  const [email, setEmail] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [isSubmitted, setIsSubmitted] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    setIsSubmitted(true)
    setIsLoading(false)
  }

  if (isSubmitted) {
    return (
      <motion.div
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.5 }}
        className="text-center"
      >
        {/* Mobile Logo */}
        <div className="lg:hidden flex justify-center mb-8">
          <Link href="/" className="flex items-center space-x-3">
            <div className="w-12 h-12 bg-gradient-to-br from-accent-500 to-accent-700 rounded-xl flex items-center justify-center">
              <span className="font-display font-bold text-2xl text-white">S</span>
            </div>
            <span className="font-display font-bold text-2xl">
              Shadow <span className="text-accent-500">OT</span>
            </span>
          </Link>
        </div>

        <div className="w-20 h-20 bg-green-500/10 rounded-full flex items-center justify-center mx-auto mb-6">
          <CheckCircle className="w-10 h-10 text-green-500" />
        </div>
        
        <h1 className="font-display text-3xl font-bold mb-3">Check Your Email</h1>
        <p className="text-shadow-400 mb-8 max-w-sm mx-auto">
          We've sent password reset instructions to{' '}
          <span className="text-white font-medium">{email}</span>
        </p>

        <div className="space-y-4">
          <p className="text-shadow-500 text-sm">
            Didn't receive the email? Check your spam folder or{' '}
            <button 
              onClick={() => setIsSubmitted(false)} 
              className="text-accent-500 hover:text-accent-400"
            >
              try again
            </button>
          </p>

          <Link
            href="/login"
            className="inline-flex items-center space-x-2 text-accent-500 hover:text-accent-400 font-medium transition-colors"
          >
            <ArrowLeft className="w-4 h-4" />
            <span>Back to Sign In</span>
          </Link>
        </div>
      </motion.div>
    )
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
    >
      {/* Mobile Logo */}
      <div className="lg:hidden flex justify-center mb-8">
        <Link href="/" className="flex items-center space-x-3">
          <div className="w-12 h-12 bg-gradient-to-br from-accent-500 to-accent-700 rounded-xl flex items-center justify-center">
            <span className="font-display font-bold text-2xl text-white">S</span>
          </div>
          <span className="font-display font-bold text-2xl">
            Shadow <span className="text-accent-500">OT</span>
          </span>
        </Link>
      </div>

      <Link 
        href="/login" 
        className="inline-flex items-center space-x-2 text-shadow-400 hover:text-white transition-colors mb-8"
      >
        <ArrowLeft className="w-4 h-4" />
        <span>Back to Sign In</span>
      </Link>

      <div className="mb-8">
        <h1 className="font-display text-3xl font-bold mb-2">Forgot Password?</h1>
        <p className="text-shadow-400">
          No worries! Enter your email and we'll send you reset instructions.
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-5">
        {/* Email */}
        <div>
          <label htmlFor="email" className="block text-sm font-medium text-shadow-300 mb-2">
            Email Address
          </label>
          <div className="relative">
            <Mail className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              id="email"
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full pl-12 pr-4 py-3 bg-shadow-900 border border-shadow-700 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 focus:ring-1 focus:ring-accent-500 transition-all"
              placeholder="warrior@shadowot.com"
            />
          </div>
        </div>

        {/* Submit Button */}
        <button
          type="submit"
          disabled={isLoading}
          className="w-full btn-primary flex items-center justify-center space-x-2 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isLoading ? (
            <>
              <Loader2 className="w-5 h-5 animate-spin" />
              <span>Sending...</span>
            </>
          ) : (
            <span>Send Reset Instructions</span>
          )}
        </button>
      </form>

      {/* Additional Help */}
      <div className="mt-8 p-4 bg-shadow-900/50 rounded-xl border border-shadow-800">
        <p className="text-shadow-400 text-sm">
          <strong className="text-white">Need help?</strong> If you're having trouble accessing your account, 
          contact our support team at{' '}
          <a href="mailto:support@shadow-ot.com" className="text-accent-500 hover:text-accent-400">
            support@shadow-ot.com
          </a>
        </p>
      </div>
    </motion.div>
  )
}

