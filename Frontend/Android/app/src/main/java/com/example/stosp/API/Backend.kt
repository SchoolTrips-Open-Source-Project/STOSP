package com.example.stosp.API

import QuotesApi
import com.example.stosp.API.Models.QuoteList

object Backend {
    suspend fun getQuotes(): QuoteList? {
        val quotesApi = RetrofitHelper.getInstance().create(QuotesApi::class.java)
        return quotesApi.getQuotes().body();
    }


}