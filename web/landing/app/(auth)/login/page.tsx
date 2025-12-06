'use client'

import { useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import { motion } from 'framer-motion'
import { Mail, Lock, Eye, EyeOff, Loader2, Wallet, ArrowRight } from 'lucide-react'
import { ConnectButton } from '@rainbow-me/rainbowkit'

export default function LoginPage() {
  const router = useRouter()
  const [showPassword, setShowPassword] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    rememberMe: false,
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    // Redirect to dashboard after login
    router.push('/dashboard')
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

      <div className="text-center mb-8">
        <h1 className="font-display text-3xl font-bold mb-2">Welcome Back</h1>
        <p className="text-shadow-400">Sign in to continue your adventure</p>
      </div>

      {/* Wallet Connect Option */}
      <div className="mb-6">
        <ConnectButton.Custom>
          {({ account, chain, openConnectModal, mounted }) => {
            const connected = mounted && account && chain
            return (
              <button
                onClick={openConnectModal}
                className="w-full flex items-center justify-center space-x-3 py-3 px-4 bg-gradient-to-r from-purple-500/10 to-blue-500/10 hover:from-purple-500/20 hover:to-blue-500/20 border border-purple-500/30 rounded-xl transition-all duration-300 group"
              >
                <Wallet className="w-5 h-5 text-purple-400" />
                <span className="text-white font-medium">
                  {connected ? `Connected: ${account.displayName}` : 'Connect Wallet'}
                </span>
                <ArrowRight className="w-4 h-4 text-purple-400 opacity-0 group-hover:opacity-100 transition-opacity" />
              </button>
            )
          }}
        </ConnectButton.Custom>
      </div>

      {/* Divider */}
      <div className="relative my-8">
        <div className="absolute inset-0 flex items-center">
          <div className="w-full border-t border-shadow-700"></div>
        </div>
        <div className="relative flex justify-center text-sm">
          <span className="px-4 bg-shadow-950 text-shadow-500">or continue with email</span>
        </div>
      </div>

      {/* Login Form */}
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
              value={formData.email}
              onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              className="w-full pl-12 pr-4 py-3 bg-shadow-900 border border-shadow-700 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 focus:ring-1 focus:ring-accent-500 transition-all"
              placeholder="warrior@shadowot.com"
            />
          </div>
        </div>

        {/* Password */}
        <div>
          <label htmlFor="password" className="block text-sm font-medium text-shadow-300 mb-2">
            Password
          </label>
          <div className="relative">
            <Lock className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              id="password"
              type={showPassword ? 'text' : 'password'}
              required
              value={formData.password}
              onChange={(e) => setFormData({ ...formData, password: e.target.value })}
              className="w-full pl-12 pr-12 py-3 bg-shadow-900 border border-shadow-700 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 focus:ring-1 focus:ring-accent-500 transition-all"
              placeholder="••••••••••"
            />
            <button
              type="button"
              onClick={() => setShowPassword(!showPassword)}
              className="absolute right-4 top-1/2 -translate-y-1/2 text-shadow-500 hover:text-white transition-colors"
            >
              {showPassword ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
            </button>
          </div>
        </div>

        {/* Remember Me & Forgot Password */}
        <div className="flex items-center justify-between">
          <label className="flex items-center">
            <input
              type="checkbox"
              checked={formData.rememberMe}
              onChange={(e) => setFormData({ ...formData, rememberMe: e.target.checked })}
              className="w-4 h-4 bg-shadow-900 border border-shadow-700 rounded text-accent-500 focus:ring-accent-500 focus:ring-offset-0"
            />
            <span className="ml-2 text-sm text-shadow-400">Remember me</span>
          </label>
          <Link
            href="/forgot-password"
            className="text-sm text-accent-500 hover:text-accent-400 transition-colors"
          >
            Forgot password?
          </Link>
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
              <span>Signing in...</span>
            </>
          ) : (
            <span>Sign In</span>
          )}
        </button>
      </form>

      {/* Sign Up Link */}
      <p className="text-center text-shadow-400 mt-8">
        Don't have an account?{' '}
        <Link href="/register" className="text-accent-500 hover:text-accent-400 font-medium transition-colors">
          Create one now
        </Link>
      </p>
    </motion.div>
  )
}

