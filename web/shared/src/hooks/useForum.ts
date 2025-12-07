import { useQuery, useMutation, useQueryClient, useInfiniteQuery } from '@tanstack/react-query'
import { forumApi } from '../api/endpoints'
import { ForumThread } from '../types'
import { useNotificationStore } from '../stores/notificationStore'

// Query keys
export const forumKeys = {
  all: ['forum'] as const,
  categories: () => [...forumKeys.all, 'categories'] as const,
  category: (slug: string) => [...forumKeys.categories(), slug] as const,
  threads: () => [...forumKeys.all, 'threads'] as const,
  threadList: (filters: ThreadFilters) => [...forumKeys.threads(), filters] as const,
  thread: (slug: string) => [...forumKeys.threads(), slug] as const,
  posts: (threadId: string) => [...forumKeys.all, 'posts', threadId] as const,
  search: (query: string) => [...forumKeys.all, 'search', query] as const,
}

export interface ThreadFilters {
  categoryId?: string
  sort?: 'latest' | 'popular' | 'oldest'
}

// Fetch forum categories
export function useForumCategories() {
  return useQuery({
    queryKey: forumKeys.categories(),
    queryFn: () => forumApi.getCategories(),
    staleTime: 1000 * 60 * 10, // 10 minutes
  })
}

// Fetch single category
export function useForumCategory(slug: string) {
  return useQuery({
    queryKey: forumKeys.category(slug),
    queryFn: () => forumApi.getCategoryBySlug(slug),
    enabled: !!slug,
  })
}

// Fetch threads
export function useForumThreads(filters: ThreadFilters = {}, page = 1, pageSize = 20) {
  return useQuery({
    queryKey: [...forumKeys.threadList(filters), page, pageSize],
    queryFn: () => forumApi.getThreads({ ...filters, page, pageSize }),
    staleTime: 1000 * 60, // 1 minute
  })
}

// Infinite scroll threads
export function useInfiniteForumThreads(filters: ThreadFilters = {}, pageSize = 20) {
  return useInfiniteQuery({
    queryKey: forumKeys.threadList(filters),
    queryFn: ({ pageParam = 1 }) => forumApi.getThreads({ ...filters, page: pageParam, pageSize }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) => {
      if (lastPage.page < lastPage.totalPages) {
        return lastPage.page + 1
      }
      return undefined
    },
    staleTime: 1000 * 60,
  })
}

// Fetch single thread
export function useForumThread(slug: string) {
  return useQuery({
    queryKey: forumKeys.thread(slug),
    queryFn: () => forumApi.getThreadBySlug(slug),
    enabled: !!slug,
  })
}

// Fetch thread posts with pagination
export function useForumPosts(threadId: string, page = 1, pageSize = 20) {
  return useQuery({
    queryKey: [...forumKeys.posts(threadId), page, pageSize],
    queryFn: () => forumApi.getPosts(threadId, { page, pageSize }),
    enabled: !!threadId,
    staleTime: 1000 * 30, // 30 seconds
  })
}

// Infinite scroll posts
export function useInfiniteForumPosts(threadId: string, pageSize = 20) {
  return useInfiniteQuery({
    queryKey: forumKeys.posts(threadId),
    queryFn: ({ pageParam = 1 }) => forumApi.getPosts(threadId, { page: pageParam, pageSize }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) => {
      if (lastPage.page < lastPage.totalPages) {
        return lastPage.page + 1
      }
      return undefined
    },
    enabled: !!threadId,
    staleTime: 1000 * 30,
  })
}

// Search forum
export function useForumSearch(query: string, page = 1, pageSize = 20) {
  return useQuery({
    queryKey: [...forumKeys.search(query), page, pageSize],
    queryFn: () => forumApi.search(query, { page, pageSize }),
    enabled: query.length >= 3,
    staleTime: 1000 * 60,
  })
}

// Create thread mutation
export function useCreateThread() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (data: {
      categoryId: string
      title: string
      content: string
      tags?: string[]
    }) => forumApi.createThread(data),
    onSuccess: (newThread) => {
      queryClient.invalidateQueries({ queryKey: forumKeys.threads() })
      success('Thread Created', `Your thread "${newThread.title}" has been posted.`)
    },
    onError: (err: Error) => {
      error('Failed to Create Thread', err.message)
    },
  })
}

// Update thread mutation
export function useUpdateThread() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ 
      id, 
      data 
    }: { 
      id: string
      data: Partial<Pick<ForumThread, 'title' | 'pinned' | 'locked'>>
    }) => forumApi.updateThread(id, data),
    onSuccess: (updatedThread) => {
      queryClient.invalidateQueries({ queryKey: forumKeys.threads() })
      queryClient.setQueryData(forumKeys.thread(updatedThread.slug), updatedThread)
      success('Thread Updated', 'The thread has been updated.')
    },
    onError: (err: Error) => {
      error('Failed to Update Thread', err.message)
    },
  })
}

// Delete thread mutation
export function useDeleteThread() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (id: string) => forumApi.deleteThread(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: forumKeys.threads() })
      success('Thread Deleted', 'The thread has been deleted.')
    },
    onError: (err: Error) => {
      error('Failed to Delete Thread', err.message)
    },
  })
}

// Create post mutation
export function useCreatePost() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ 
      threadId, 
      content, 
      quoteIds 
    }: { 
      threadId: string
      content: string
      quoteIds?: string[]
    }) => forumApi.createPost(threadId, content, quoteIds),
    onSuccess: (_, { threadId }) => {
      queryClient.invalidateQueries({ queryKey: forumKeys.posts(threadId) })
      success('Reply Posted', 'Your reply has been posted.')
    },
    onError: (err: Error) => {
      error('Failed to Post Reply', err.message)
    },
  })
}

// Update post mutation
export function useUpdatePost() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ postId, content }: { postId: string; content: string }) => 
      forumApi.updatePost(postId, content),
    onSuccess: (updatedPost) => {
      queryClient.invalidateQueries({ queryKey: forumKeys.posts(updatedPost.threadId) })
      success('Post Updated', 'Your post has been updated.')
    },
    onError: (err: Error) => {
      error('Failed to Update Post', err.message)
    },
  })
}

// Delete post mutation
export function useDeletePost() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (postId: string) => forumApi.deletePost(postId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: forumKeys.all })
      success('Post Deleted', 'The post has been deleted.')
    },
    onError: (err: Error) => {
      error('Failed to Delete Post', err.message)
    },
  })
}

// React to post mutation
export function useReactToPost() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ 
      postId, 
      reaction 
    }: { 
      postId: string
      reaction: 'like' | 'helpful' | 'funny'
    }) => forumApi.reactToPost(postId, reaction),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: forumKeys.all })
    },
  })
}


