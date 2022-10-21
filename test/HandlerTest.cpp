//
// Created by moosavi on 10/21/22.
//

#include "../include/Handler.h"
#include <gtest/gtest.h>
#include <gmock/gmock.h>

class HandlerTest : public testing::Test {
protected:
    void SetUp() override {
        this->handler = new Handler();
    }

    void TearDown() override {
        delete this->handler;
    }

    Handler *handler{};
};

